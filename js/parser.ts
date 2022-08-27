import {FlatState, XY} from '../pkg/index';

export async function parseDemo(bytes: Uint8Array, progressCallback : (progress: number) => void): Promise<ParsedDemo> {
    let m = await import("../pkg/index.js");
    const state = m.parse_demo(bytes, progressCallback);

    let playerCount = state.player_count;
    let buildingCount = state.building_count;
    let boundaries = state.boundaries;
    let interval_per_tick = state.interval_per_tick;
    let tickCount = state.tick_count;
    let kill_ticks = m.get_kill_ticks(state);
    let attackers = m.get_attacker_ids(state);
    let assisters = m.get_assister_ids(state);
    let victims = m.get_victim_ids(state);

    let playerInfo = [];

    for (let i = 0; i < playerCount; i++) {
        playerInfo.push({
            name: m.get_player_name(state, i),
            steamId: m.get_player_steam_id(state, i),
            entityId: m.get_player_entity_id(state, i),
            userId: m.get_player_user_id(state, i),
        })
    }

    let kills = [];
    for (let i = 0; i < kill_ticks.length; i++) {
        kills.push({
            tick: kill_ticks[i],
            attacker: attackers[i],
            assister: assisters[i],
            victim: victims[i],
            weapon: m.get_weapon(state, i),
        })
    }

    let map = m.get_map(state);
    let data = m.get_data(state);

    return new ParsedDemo(
        playerCount,
        buildingCount,
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
        data,
        kills,
        playerInfo,
        tickCount,
    );
}

export interface PlayerInfo {
    entityId: number,
    name: string,
    steamId: string,
    userId: number,
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

export enum BuildingType {
    TeleporterEntrance = 0,
    TeleporterExit = 1,
    Dispenser = 2,
    Level1Sentry = 3,
    Level2Sentry = 4,
    Level3Sentry = 5,
    MiniSentry = 6,
    Unknown = 7,
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
    info: PlayerInfo,
    charge: number,
}

export interface BuildingState {
    position: {
        x: number,
        y: number
    },
    angle: number,
    health: number,
    level: number,
    team: Team,
    buildingType: BuildingType,
}

export interface Header {
    interval_per_tick: number,
    map: string
}

export interface Kill {
    tick: number,
    attacker: number,
    assister: number,
    victim: number,
    weapon: string,
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
    public readonly buildingCount: number;
    public readonly world: WorldBoundaries;
    public readonly data: Uint8Array;
    private readonly header: Header;
    public readonly tickCount: number;
    public readonly kills: Kill[];
    public readonly playerInfo: PlayerInfo[];

    constructor(playerCount: number, buildingCount: number, world: WorldBoundaries, header: Header, data: Uint8Array, kills: Kill[], playerInfo: PlayerInfo[], tickCount: number) {
        this.playerCount = playerCount;
        this.buildingCount = buildingCount;
        this.world = world;
        this.header = header;
        this.data = data;
        this.kills = kills;
        this.playerInfo = playerInfo;
        this.tickCount = tickCount;
    }

    getPlayer(tick: number, playerIndex: number): PlayerState {
        if (playerIndex >= this.playerCount) {
            throw new Error("Player out of bounds");
        }

        const base = ((playerIndex * this.tickCount) + tick) * PLAYER_PACK_SIZE;
        return unpackPlayer(this.data, base, this.world, this.playerInfo[playerIndex]);
    }

    getBuilding(tick: number, buildingIndex: number): BuildingState {
        if (buildingIndex >= this.buildingCount) {
            throw new Error("Building out of bounds");
        }

        const base = (this.playerCount * this.tickCount * PLAYER_PACK_SIZE) + ((buildingIndex * this.tickCount) + tick) * BUILDING_PACK_SIZE;
        return unpackBuilding(this.data, base, this.world);
    }
}

const PLAYER_PACK_SIZE = 8;
const BUILDING_PACK_SIZE = 7;

function unpackPlayer(bytes: Uint8Array, base: number, world: WorldBoundaries, info: PlayerInfo): PlayerState {
    const x = unpack_f32(bytes[base] + (bytes[base + 1] << 8), world.boundary_min.x, world.boundary_max.x);
    const y = unpack_f32(bytes[base + 2] + (bytes[base + 3] << 8), world.boundary_min.y, world.boundary_max.y);
    const team_class_health = bytes[base + 4] + (bytes[base + 5] << 8);
    const angle = unpack_angle(bytes[base + 6]);
    const health = team_class_health & 1013;
    const team = (team_class_health >> 14) as Team;
    const playerClass = ((team_class_health >> 10) & 15) as Class;
    const charge = bytes[base + 7];

    return {
        position: {x, y},
        angle,
        health,
        team,
        playerClass,
        info,
        charge
    }
}

function unpackBuilding(bytes: Uint8Array, base: number, world: WorldBoundaries): BuildingState {
    const x = unpack_f32(bytes[base] + (bytes[base + 1] << 8), world.boundary_min.x, world.boundary_max.x);
    const y = unpack_f32(bytes[base + 2] + (bytes[base + 3] << 8), world.boundary_min.y, world.boundary_max.y);
    const team_type_health = bytes[base + 4] + (bytes[base + 5] << 8);
    const angle = unpack_angle(bytes[base + 6]);
    const health = team_type_health & 1013;
    const team = (((team_type_health >> 13) & 1) === 0) ? Team.Blue : Team.Red;
    const level = (team_type_health >> 14);
    const buildingType = ((team_type_health >> 10) & 7) as BuildingType;

    return {
        position: {x, y},
        angle,
        health,
        team,
        buildingType,
        level,
    }
}
