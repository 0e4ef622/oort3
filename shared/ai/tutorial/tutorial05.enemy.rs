use oort_api::prelude::*;

const SPEED: f64 = 200.0;

pub struct Ship {
    target: Vec2,
    rng: oorandom::Rand64,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            target: position(),
            rng: oorandom::Rand64::new(seed()),
        }
    }

    pub fn tick(&mut self) {
        if (self.target - position()).length() < 200.0 {
            self.target = vec2(1000.0, 0.0).rotate(self.rng.rand_float() * TAU);
        }

        debug_line(position(), self.target, 0xffffff);

        let target_velocity = (self.target - position()).normalize() * SPEED;
        accelerate((target_velocity - velocity()) * 1e6);
        turn_to((target_velocity - velocity()).angle());
    }
}

fn turn_to(target_heading: f64) {
    let heading_error = angle_diff(heading(), target_heading);
    turn(3.0 * heading_error);
}
