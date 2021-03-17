use rand::Rng;
use std::f64;

#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct Circle {
    pub position: Position,
    pub radius: f64,
}

impl Circle {
    pub fn rand(position: Position) -> Self {
        let mut rng = rand::thread_rng();
        Circle {
            position,
            radius: rng.gen_range(0.1..20.0),
        }
    }

    pub fn update(&mut self, view_window: &ViewWindow, max_position_delta: f64) {
        let mut rng = rand::thread_rng();

        self.position.x = rng.gen_range(
            f64::max(view_window.x_min, self.position.x - max_position_delta)
                ..f64::min(view_window.x_max, self.position.x + max_position_delta),
        );

        self.position.y = rng.gen_range(
            f64::max(view_window.y_min, self.position.y - max_position_delta)
                ..f64::min(view_window.y_max, self.position.y + max_position_delta),
        )
    }
}

pub struct ViewWindow {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

impl ViewWindow {
    pub fn random_position(&self) -> Position {
        let mut rng = rand::thread_rng();
        Position {
            x: rng.gen_range(self.x_min..self.x_max),
            y: rng.gen_range(self.y_min..self.y_max),
        }
    }
}
