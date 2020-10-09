import {FlatState, XY} from '../pkg/index';

export async function parseDemo(bytes: Uint8Array): Promise<ParsedDemo> {
    let m = await import("../pkg/index.js");
    const state = m.parse_demo(bytes);

    let playerCount = state.player_count;
    let boundaries = state.boundaries;
    let interval_per_tick = state.interval_per_tick;
    let map = m.get_map(state);
    let data = m.get_data(state);

    return new ParsedDemo(
        playerCount,
        {
            boundary_min: {
                x: boundaries.boundary_min.x,
                y: boundaries.boundary_min.y,
            },
            boundary_max: {
                x: boundaries.boundary_max.x,
                y: boundaries.boundary_max.y,
            }
        },
        {
            map,
            interval_per_tick
        },
        data
    );
}

export enum Team {
    Other = 0,
    Spectator = 1,
    Red = 2,
    Blue = 3,
}

export enum Class {
    Other = 0,
    Scout = 1,
    Sniper = 2,
    Solder = 3,
    Demoman = 4,
    Medic = 5,
    Heavy = 6,
    Pyro = 7,
    Spy = 8,
    Engineer = 9,
}

export interface WorldBoundaries {
    boundary_min: {
        x: number,
        y: number
    },
    boundary_max: {
        x: number,
        y: number
    }
}

export interface PlayerState {
    position: {
        x: number,
        y: number
    },
    angle: number,
    health: number,
    team: Team,
    playerClass: Class,
}

export interface Header {
    interval_per_tick: number,
    map: string
}

function unpack_f32(val: number, min: number, max: number): number {
    const ratio = val / (Math.pow(2, 16) - 1);
    return ratio * (max - min) + min;
}

function unpack_angle(val: number): number {
    const ratio = val / (Math.pow(2, 8) - 1);
    return ratio * 360;
}

export class ParsedDemo {
    public readonly playerCount: number;
    public readonly world: WorldBoundaries;
    public readonly data: Uint8Array;
    private readonly header: Header;
    public readonly tickCount: number;

    constructor(playerCount: number, world: WorldBoundaries, header: Header, data: Uint8Array) {
        this.playerCount = playerCount;
        this.world = world;
        this.header = header;
        this.data = data;
        this.tickCount = data.length / playerCount / PACK_SIZE;
    }

    getPlayer(tick: number, playerIndex: number): PlayerState {
        if (playerIndex >= this.playerCount) {
            throw new Error("Player out of bounds");
        }

        const base = ((playerIndex * this.tickCount) + tick) * PACK_SIZE;
        return unpackPlayer(this.data, base, this.world);
    }
}

const PACK_SIZE = 7;

function unpackPlayer(bytes: Uint8Array, base: number, world: WorldBoundaries): PlayerState {
    const x = unpack_f32(bytes[base] + (bytes[base + 1] << 8), world.boundary_min.x, world.boundary_max.x);
    const y = unpack_f32(bytes[base + 2] + (bytes[base + 3] << 8), world.boundary_min.y, world.boundary_max.y);
    const team_class_health = bytes[base + 4] + (bytes[base + 5] << 8);
    const angle = unpack_angle(bytes[base + 6]);
    const health = team_class_health & 1013;
    const team = (team_class_health >> 14) as Team;
    const playerClass = ((team_class_health >> 10) & 15) as Class;

    return {
        position: {x, y},
        angle,
        health,
        team,
        playerClass
    }
}