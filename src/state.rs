use std::ops::Index;
use tf_demo_parser::demo::parser::gamestateanalyser::{Class, GameState, Team};
use tf_demo_parser::demo::vector::VectorXY;

macro_rules! log {
    ($($arg:tt)*) => (web_sys::console::log_1(&wasm_bindgen::prelude::JsValue::from(format!($($arg)*))))
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Angle(u16);

impl From<f32> for Angle {
    fn from(val: f32) -> Self {
        Angle(val.rem_euclid(360.0) as u16)
    }
}

impl From<Angle> for u16 {
    fn from(val: Angle) -> Self {
        val.0
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
}

#[derive(Debug, Default, Clone)]
pub struct ParsedPlayer {
    position: Vec<VectorXY>,
    angle: SparseVec<Angle, 1>,
    health: SparseVec<u16, 4>,
    team: SparseVec<Team, 128>,
    class: SparseVec<Class, 128>,
}

#[derive(Debug, Default, Clone)]
pub struct PlayerState {
    position: VectorXY,
    angle: Angle,
    health: u16,
    team: Team,
    class: Class,
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
}

#[derive(Debug, Default, Clone)]
pub struct SparseVec<T: Default, const N: usize> {
    inner: Vec<T>,
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
