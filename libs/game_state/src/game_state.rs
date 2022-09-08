use common::*;

use xs::{Xs, Seed};

pub type X = u8;
pub type Y = u8;

#[derive(Clone, Default)]
pub struct Splat {
    pub kind: Card,
    pub x: X,
    pub y: Y,
}

#[derive(Clone, Default)]
pub struct GameState {
    pub rng: Xs,
    pub splats: Vec<Splat>,
}

impl GameState {
    pub fn new(seed: Seed) -> GameState {
        let rng = xs::from_seed(seed);

        GameState {
            rng,
            .. <_>::default()
        }
    }

    pub fn add_splat(&mut self) {
        let rng = &mut self.rng;

        let kind: Card = gen_card(rng);
        let x = xs::range(rng, 0..screen::WIDTH as _) as X;
        let y = xs::range(rng, 0..screen::HEIGHT as _) as Y;

        self.splats.push(Splat {
            kind,
            x,
            y,
        });
    }
}
