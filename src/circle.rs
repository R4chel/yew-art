use rand::Rng;
use std::f64;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct RangeConfig {
    min: f64,
    max: f64,
    delta: f64,
}

impl RangeConfig {
    fn rand(&self) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.min..self.max)
    }

    fn upate_value(&self, value: f64) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(
            f64::max(self.min, value - self.delta)..f64::min(self.max, value + self.delta),
        )
    }
}

#[derive(Clone)]
pub struct ColorConfig {
    h: RangeConfig,
    s: RangeConfig,
    l: RangeConfig,
}

impl ColorConfig {
    pub fn default() -> Self {
        ColorConfig {
            h: RangeConfig {
                delta: 5.0,
                min: 0.0,
                max: 360.0,
            },

            s: RangeConfig {
                delta: 0.1,
                min: 0.0,
                max: 1.0,
            },

            l: RangeConfig {
                delta: 0.1,
                min: 0.0,
                max: 1.0,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    h: f64,
    s: f64,
    l: f64,
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hsl({:.2}, {:.2}%, {:.2}%)",
            self.h,
            self.s * 100.0,
            self.l * 100.0
        )
    }
}
impl Color {
    fn rand(color_config: &ColorConfig) -> Self {
        Color {
            h: color_config.h.rand(),
            s: color_config.s.rand(),
            l: color_config.l.rand(),
        }
    }

    fn update(&mut self, color_config: &ColorConfig) {
        self.h = color_config.h.upate_value(self.h);
        self.s = color_config.s.upate_value(self.s);
        self.l = color_config.l.upate_value(self.l);
    }
}

#[derive(Clone)]
pub struct Circle {
    pub position: Position,
    pub radius: f64,
    pub color: Color,
}

impl Circle {
    pub fn rand(color_config: &ColorConfig, position: Position) -> Self {
        let mut rng = rand::thread_rng();
        Circle {
            position,
            radius: rng.gen_range(0.1..20.0),
            color: Color::rand(color_config),
        }
    }

    pub fn update(
        &mut self,
        view_window: &ViewWindow,
        max_position_delta: f64,
        color_config: &ColorConfig,
    ) {
        let mut rng = rand::thread_rng();

        self.position.x = rng.gen_range(
            f64::max(view_window.x_min, self.position.x - max_position_delta)
                ..f64::min(view_window.x_max, self.position.x + max_position_delta),
        );

        self.position.y = rng.gen_range(
            f64::max(view_window.y_min, self.position.y - max_position_delta)
                ..f64::min(view_window.y_max, self.position.y + max_position_delta),
        );
        self.color.update(&color_config)
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
