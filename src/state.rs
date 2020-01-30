use std::ops::Index;
use tf_demo_parser::demo::parser::gamestateanalyser::{Class, GameState, Team, World};
use tf_demo_parser::demo::vector::{Vector, VectorXY};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Angle(u8);

impl From<f32> for Angle {
    fn from(val: f32) -> Self {
        let ratio = val.rem_euclid(360.0) / 360.0;
        Angle((ratio * u8::max_value() as f32) as u8)
    }
}

impl From<Angle> for f32 {
    fn from(val: Angle) -> Self {
        let ratio = val.0 as f32 / u8::max_value() as f32;
        ratio * 360.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct ParsedDemo {
    tick: usize,
    pub players: Vec<ParsedPlayer>,
}

impl ParsedDemo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_state(&mut self, game_state: &GameState) {
        for (index, player) in game_state.players.iter().enumerate() {
            if let None = self.players.get(index) {
                let mut new_player = ParsedPlayer::default();
                // backfill with defaults
                new_player.resize(self.tick);
                self.players.push(new_player)
            };

            let parsed_player = &mut self.players[index];
            parsed_player.push(
                self.tick,
                player.position.into(),
                player.view_angle.into(),
                player.health,
                player.team,
                player.class,
            );
        }
        self.tick += 1;
    }

    pub fn size(&self) -> usize {
        self.players
            .iter()
            .fold(0, |size, player| size + player.size())
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PlayerState {
    position: VectorXY,
    angle: Angle,
    health: u16,
    team: Team,
    class: Class,
}

impl PlayerState {
    pub fn pack(&self, world: &World) -> [u8; 8] {
        // for the purpose of viewing the demo in the browser we dont really need high accuracy for
        // position or angle, so we save a bunch of space by truncating those down to half the number
        // of bits
        fn pack_f32(val: f32, min: f32, max: f32) -> u16 {
            let ratio = (val - min) / (max - min);
            (ratio * u16::max_value() as f32) as u16
        }

        let x = pack_f32(self.position.x, world.boundary_min.x, world.boundary_max.x).to_le_bytes();
        let y = pack_f32(self.position.y, world.boundary_min.y, world.boundary_max.y).to_le_bytes();
        let team_and_class = ((self.team as u8) << 4) + self.class as u8;
        let health_bytes = self.health.to_le_bytes();

        [
            x[0],
            x[1],
            y[0],
            y[1],
            health_bytes[0],
            health_bytes[1],
            self.angle.0,
            team_and_class,
        ]
    }

    pub fn unpack(bytes: [u8; 8], world: &World) -> Self {
        fn unpack_f32(val: u16, min: f32, max: f32) -> f32 {
            let ratio = val as f32 / (u16::max_value() as f32);
            ratio * (max - min) + min
        }

        let x = unpack_f32(
            u16::from_le_bytes([bytes[0], bytes[1]]),
            world.boundary_min.x,
            world.boundary_max.x,
        );
        let y = unpack_f32(
            u16::from_le_bytes([bytes[2], bytes[3]]),
            world.boundary_min.y,
            world.boundary_max.y,
        );
        let health = u16::from_le_bytes([bytes[4], bytes[5]]);
        let angle = Angle(bytes[6]);
        let team = Team::new(bytes[7] >> 4);
        let class = Class::new(bytes[7] & 15);

        PlayerState {
            position: VectorXY { x, y },
            angle,
            health,
            team,
            class,
        }
    }
}

#[test]
fn test_packing() {
    let world = World {
        boundary_max: Vector {
            x: 10000.0,
            y: 10000.0,
            z: 100.0,
        },
        boundary_min: Vector {
            x: -10000.0,
            y: -10000.0,
            z: -100.0,
        },
    };

    let input = PlayerState {
        position: VectorXY {
            x: 100.0,
            y: -5000.0,
        },
        angle: Angle::from(213.0),
        health: 250,
        team: Team::Blue,
        class: Class::Demoman,
    };

    let bytes = input.pack(&world);

    let unpacked = PlayerState::unpack(bytes, &world);
    assert_eq!(input.angle, unpacked.angle);
    assert_eq!(input.health, unpacked.health);
    assert_eq!(input.class, unpacked.class);
    assert_eq!(input.team, unpacked.team);

    assert!(f32::abs(input.position.x - unpacked.position.x) < 0.5);
    assert!(f32::abs(input.position.y - unpacked.position.y) < 0.5);
}

#[derive(Debug, Default, Clone)]
pub struct ParsedPlayer {
    position: Vec<VectorXY>,
    angle: SparseVec<Angle, 1>,
    health: SparseVec<u16, 4>,
    team: SparseVec<Team, 128>,
    class: SparseVec<Class, 128>,
}

impl ParsedPlayer {
    fn push(
        &mut self,
        index: usize,
        position: VectorXY,
        angle: Angle,
        health: u16,
        team: Team,
        class: Class,
    ) {
        debug_assert!(self.position.len() == index);
        self.position.push(position);

        self.angle.push_index(index, angle);
        self.health.push_index(index, health);
        self.team.push_index(index, team);
        self.class.push_index(index, class);
    }

    fn resize(&mut self, size: usize) {
        self.position.resize_with(size, || VectorXY::default());
        self.angle.resize(size);
        self.health.resize(size);
        self.team.resize(size);
        self.class.resize(size);
    }

    pub fn len(&self) -> usize {
        self.position.len()
    }

    pub fn get(&self, index: usize) -> PlayerState {
        PlayerState {
            position: self.position[index],
            angle: self.angle[index],
            health: self.health[index],
            team: self.team[index],
            class: self.class[index],
        }
    }

    pub fn size(&self) -> usize {
        self.position.len() * std::mem::size_of::<VectorXY>()
            + self.team.size()
            + self.class.size()
            + self.health.size()
            + self.angle.size()
    }
}

#[derive(Debug, Default, Clone)]
pub struct SparseVec<T: Default, const N: usize> {
    inner: Vec<T>,
}

impl<T: Default, const N: usize> SparseVec<T, N> {
    pub fn size(&self) -> usize {
        self.inner.len() * std::mem::size_of::<T>()
    }
}

impl<T: Default, const N: usize> SparseVec<T, N> {
    fn push_index(&mut self, index: usize, val: T) {
        if index % N == 0 {
            self.inner.push(val)
        }
    }

    fn resize(&mut self, size: usize) {
        self.inner.resize_with(size / N, Default::default)
    }
}

impl<T: Default, const N: usize> Index<usize> for SparseVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(index / N)
    }
}
