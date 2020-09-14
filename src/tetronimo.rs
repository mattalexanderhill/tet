use crate::BLOCK_WIDTH;
use bevy::prelude::*;
use rand::seq::SliceRandom;

#[derive(Copy, Clone)]
enum Shape {I, O, T, J, L, S, Z,}

const SHAPES: [Shape; 7] = [
    Shape::I,
    Shape::O,
    Shape::T,
    Shape::J,
    Shape::L,
    Shape::S,
    Shape::Z,
];

enum Orient {
    North,
    East,
    South,
    West,
}

pub struct Tetronimo {
    shape: Shape,
    orient: Orient,
}

impl Tetronimo {
    pub fn random() -> Self {
        let mut rng = &mut rand::thread_rng();
        let shape = SHAPES
            .choose(&mut rng)
            .unwrap();

        Tetronimo {
            shape: *shape,
            orient: Orient::North,
        }
    }

    pub fn size(&self) -> Vec2 {
        let unit_vec = match self.shape {
            Shape::I => Vec2::new(1.0, 4.0),
            Shape::O => Vec2::new(2.0, 2.0),
            Shape::T => Vec2::new(2.0, 3.0),
            Shape::J => Vec2::new(2.0, 4.0),
            Shape::L => Vec2::new(2.0, 4.0),
            Shape::S => Vec2::new(3.0, 2.0),
            Shape::Z => Vec2::new(3.0, 2.0),
        };
        unit_vec * BLOCK_WIDTH
    }

    pub fn color(&self) -> Color {
        match self.shape {
            Shape::I => Color::rgb(0.30, 0.44, 0.65),
            Shape::O => Color::rgb(0.95, 0.55, 0.16),
            Shape::T => Color::rgb(0.88, 0.34, 0.35),
            Shape::J => Color::rgb(0.46, 0.71, 0.69),
            Shape::L => Color::rgb(0.35, 0.63, 0.31),
            Shape::S => Color::rgb(0.69, 0.47, 0.63),
            Shape::Z => Color::rgb(0.93, 0.78, 0.28),
        }
    }
}
