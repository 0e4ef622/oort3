use super::ship::{asteroid, fighter};
use super::{
    bullet, ship, Simulation, BULLET_COLLISION_GROUP, SHIP_COLLISION_GROUP, WALL_COLLISION_GROUP,
    WORLD_SIZE,
};
use nalgebra::{Point2, Translation2, Vector4};
use rand::seq::SliceRandom;
use rand::Rng;
use rapier2d_f64::prelude::*;
use Status::Running;

#[derive(PartialEq, Debug)]
pub enum Status {
    Running,
    Finished,
}

pub struct Line {
    pub a: Point2<f64>,
    pub b: Point2<f64>,
    pub color: Vector4<f32>,
}

pub trait Scenario {
    fn name(&self) -> String;

    fn init(&mut self, sim: &mut Simulation);

    fn tick(&mut self, _: &mut Simulation) {}

    fn status(&self, _: &Simulation) -> Status {
        Running
    }

    fn lines(&self) -> Vec<Line> {
        Vec::new()
    }

    fn initial_code(&self) -> String {
        "".to_string()
    }

    fn solution(&self) -> String {
        "".to_string()
    }
}

pub fn add_walls(sim: &mut Simulation) {
    let mut make_edge = |x: f64, y: f64, a: f64| {
        let edge_length = WORLD_SIZE as f64;
        let edge_width = 10.0;
        let rigid_body = RigidBodyBuilder::new_static()
            .translation(vector![x, y])
            .rotation(a)
            .build();
        let body_handle = sim.bodies.insert(rigid_body);
        let collider = ColliderBuilder::cuboid(edge_length / 2.0, edge_width / 2.0)
            .restitution(1.0)
            .collision_groups(InteractionGroups::new(
                1 << WALL_COLLISION_GROUP,
                1 << SHIP_COLLISION_GROUP | 1 << BULLET_COLLISION_GROUP,
            ))
            .build();
        sim.colliders
            .insert_with_parent(collider, body_handle, &mut sim.bodies);
    };
    make_edge(0.0, WORLD_SIZE / 2.0, 0.0);
    make_edge(0.0, -WORLD_SIZE / 2.0, std::f64::consts::PI);
    make_edge(WORLD_SIZE / 2.0, 0.0, std::f64::consts::PI / 2.0);
    make_edge(-WORLD_SIZE / 2.0, 0.0, 3.0 * std::f64::consts::PI / 2.0);
}

pub fn load(name: &str) -> Box<dyn Scenario> {
    let scenario: Box<dyn Scenario> = match name {
        "basic" => Box::new(BasicScenario {}),
        "asteroid" => Box::new(AsteroidScenario {}),
        "bullet-stress" => Box::new(BulletStressScenario {}),
        "welcome" => Box::new(WelcomeScenario {}),
        "tutorial01" => Box::new(Tutorial01 {}),
        "tutorial02" => Box::new(Tutorial02::new()),
        "tutorial03" => Box::new(Tutorial03::new()),
        "tutorial04" => Box::new(Tutorial04::new()),
        _ => panic!("Unknown scenario"),
    };
    assert_eq!(scenario.name(), name);
    scenario
}

struct BasicScenario {}

impl Scenario for BasicScenario {
    fn name(&self) -> String {
        "basic".into()
    }

    fn init(&mut self, sim: &mut Simulation) {
        add_walls(sim);
        ship::create(sim, -100.0, 0.0, 0.0, 0.0, 0.0, fighter());
        ship::create(sim, 100.0, 0.0, 0.0, 0.0, std::f64::consts::PI, fighter());
    }

    fn status(&self, sim: &Simulation) -> Status {
        if sim.ships.iter().len() > 1 {
            Running
        } else {
            Status::Finished
        }
    }
}

struct AsteroidScenario {}

impl Scenario for AsteroidScenario {
    fn name(&self) -> String {
        "asteroid".into()
    }

    fn init(&mut self, sim: &mut Simulation) {
        let mut rng = rand::thread_rng();
        add_walls(sim);
        ship::create(sim, 0.0, 0.0, 0.0, 0.0, 0.0, fighter());

        let bound = (WORLD_SIZE / 2.0) * 0.9;
        for _ in 0..1000 {
            ship::create(
                sim,
                rng.gen_range(-bound..bound),
                rng.gen_range(-bound..bound),
                rng.gen_range(-30.0..30.0),
                rng.gen_range(-30.0..30.0),
                rng.gen_range(0.0..(2.0 * std::f64::consts::PI)),
                asteroid(rng.gen_range(0..30)),
            );
        }
    }

    fn status(&self, sim: &Simulation) -> Status {
        if sim.ships.iter().len() > 1 {
            Running
        } else {
            Status::Finished
        }
    }
}

struct BulletStressScenario {}

impl Scenario for BulletStressScenario {
    fn name(&self) -> String {
        "bullet-stress".into()
    }

    fn init(&mut self, sim: &mut Simulation) {
        let mut rng = rand::thread_rng();
        add_walls(sim);
        ship::create(sim, 0.0, 0.0, 0.0, 0.0, 0.0, fighter());

        let bound = (WORLD_SIZE / 2.0) * 0.9;
        for _ in 0..1000 {
            let s = 1000.0;
            bullet::create(
                sim,
                rng.gen_range(-bound..bound),
                rng.gen_range(-bound..bound),
                rng.gen_range(-s..s),
                rng.gen_range(-s..s),
            );
        }
    }
}

struct WelcomeScenario {}

impl Scenario for WelcomeScenario {
    fn name(&self) -> String {
        "welcome".into()
    }

    fn init(&mut self, sim: &mut Simulation) {
        let mut rng = rand::thread_rng();
        add_walls(sim);
        ship::create(sim, 0.0, 0.0, 0.0, 0.0, 0.0, fighter());
        let asteroid_variants = [1, 6, 14];
        let bound = (1000.0 / 2.0) * 0.9;
        for _ in 0..100 {
            ship::create(
                sim,
                rng.gen_range(-bound..bound),
                rng.gen_range(-bound..bound),
                rng.gen_range(-30.0..30.0),
                rng.gen_range(-30.0..30.0),
                rng.gen_range(0.0..(2.0 * std::f64::consts::PI)),
                asteroid(*asteroid_variants.choose(&mut rng).unwrap()),
            );
        }
    }

    fn initial_code(&self) -> String {
        "\
// Welcome to Oort.
// Select a scenario from the list in the top-right of the page.
// If you're new, start with \"tutorial01\"."
            .to_string()
    }
}

struct Tutorial01 {}

impl Scenario for Tutorial01 {
    fn name(&self) -> String {
        "tutorial01".into()
    }

    fn init(&mut self, sim: &mut Simulation) {
        ship::create(sim, 0.0, 0.0, 0.0, 0.0, 0.0, fighter());
        ship::create(sim, 100.0, 0.0, 0.0, 0.0, 0.1, asteroid(1));
    }

    fn status(&self, sim: &Simulation) -> Status {
        if sim.ships.iter().len() > 1 {
            Running
        } else {
            Status::Finished
        }
    }

    fn initial_code(&self) -> String {
        "\
// Tutorial 01
// Destroy the asteroid.

fn tick() {
    // Uncomment me, then press ctrl-Enter to upload the code.
    // api.fire_weapon();
}"
        .to_string()
    }

    fn solution(&self) -> String {
        "\
// Tutorial 01
// Destroy the asteroid.

fn tick() {
    // Uncomment me, then press ctrl-Enter to upload the code.
    api.fire_weapon();
}"
        .to_string()
    }
}

struct Tutorial02 {
    on_target_ticks: i32,
}

impl Tutorial02 {
    fn new() -> Self {
        Self { on_target_ticks: 0 }
    }
}

impl Scenario for Tutorial02 {
    fn name(&self) -> String {
        "tutorial02".into()
    }

    fn init(&mut self, sim: &mut Simulation) {
        ship::create(sim, 0.0, 0.0, 0.0, 0.0, 0.0, fighter());
        if let Some(&handle) = sim.ships.iter().next() {
            let c = sim.ship_controllers.get_mut(&handle);
            c.unwrap().write_target(vector![200.0, 0.0]);
        }
    }

    fn tick(&mut self, sim: &mut Simulation) {
        if let Some(&handle) = sim.ships.iter().next() {
            let ship = sim.ship(handle);
            if (ship.position().vector - Translation2::new(200.0, 0.0).vector).magnitude() < 50.0
                && ship.velocity().magnitude() < 1.0
            {
                self.on_target_ticks += 1;
            } else {
                self.on_target_ticks = 0;
            }
        }
    }

    fn status(&self, _: &Simulation) -> Status {
        if self.on_target_ticks > 120 {
            Status::Finished
        } else {
            Status::Running
        }
    }

    fn lines(&self) -> Vec<Line> {
        let mut lines = vec![];
        let center: Point2<f64> = point![200.0, 0.0];
        let n = 20;
        let r = 50.0;
        let on_target_frac = self.on_target_ticks as f64 / 120.0;
        for i in 0..n {
            let frac = (i as f64) / (n as f64);
            let angle_a = std::f64::consts::TAU * frac;
            let angle_b = std::f64::consts::TAU * (frac + 1.0 / n as f64);
            let color = if on_target_frac > frac {
                vector![0.0, 1.0, 0.0, 1.0]
            } else {
                vector![1.0, 0.0, 0.0, 1.0]
            };
            lines.push(Line {
                a: center + vector![r * angle_a.cos(), r * angle_a.sin()],
                b: center + vector![r * angle_b.cos(), r * angle_b.sin()],
                color,
            });
        }
        lines
    }

    fn initial_code(&self) -> String {
        "\
// Tutorial 02
// Fly to the target circle and stop.

fn tick() {
    api.accelerate(vec2(100.0, 0.0));
}"
        .to_string()
    }

    fn solution(&self) -> String {
        "\
// Tutorial 02
// Fly to the target circle and stop.

fn tick() {
    let acc = 100.0;
    let x = api.position().x;
    let dx = target.x - x;
    let vx = api.velocity().x;
    let margin = 10.0;
    let t = abs(vx / acc);
    let pdx = (x + vx * t + 0.5 * -acc * t*t) - target.x;
    if pdx > -margin && pdx < margin {
        api.accelerate(vec2(-vx * 10, 0.0));
    } else if pdx < -margin {
        api.accelerate(vec2(acc, 0.0));
    } else if pdx > margin {
        api.accelerate(vec2(-acc, 0.0));
    }
}
"
        .to_string()
    }
}

struct Tutorial03 {
    on_target_ticks: i32,
    target: Point2<f64>,
}

impl Tutorial03 {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let size = 500.0;
        let range = -size..size;
        Self {
            on_target_ticks: 0,
            target: point![rng.gen_range(range.clone()), rng.gen_range(range)],
        }
    }
}

impl Scenario for Tutorial03 {
    fn name(&self) -> String {
        "tutorial03".into()
    }

    fn init(&mut self, sim: &mut Simulation) {
        ship::create(sim, 0.0, 0.0, 0.0, 0.0, 0.0, fighter());
        if let Some(&handle) = sim.ships.iter().next() {
            let c = sim.ship_controllers.get_mut(&handle);
            c.unwrap().write_target(self.target.coords);
        }
    }

    fn tick(&mut self, sim: &mut Simulation) {
        if let Some(&handle) = sim.ships.iter().next() {
            let ship = sim.ship(handle);
            if (ship.position().vector - self.target.coords).magnitude() < 50.0
                && ship.velocity().magnitude() < 1.0
            {
                self.on_target_ticks += 1;
            } else {
                self.on_target_ticks = 0;
            }
        }
    }

    fn status(&self, _: &Simulation) -> Status {
        if self.on_target_ticks > 120 {
            Status::Finished
        } else {
            Status::Running
        }
    }

    fn lines(&self) -> Vec<Line> {
        let mut lines = vec![];
        let center: Point2<f64> = self.target;
        let n = 20;
        let r = 50.0;
        let on_target_frac = self.on_target_ticks as f64 / 120.0;
        for i in 0..n {
            let frac = (i as f64) / (n as f64);
            let angle_a = std::f64::consts::TAU * frac;
            let angle_b = std::f64::consts::TAU * (frac + 1.0 / n as f64);
            let color = if on_target_frac > frac {
                vector![0.0, 1.0, 0.0, 1.0]
            } else {
                vector![1.0, 0.0, 0.0, 1.0]
            };
            lines.push(Line {
                a: center + vector![r * angle_a.cos(), r * angle_a.sin()],
                b: center + vector![r * angle_b.cos(), r * angle_b.sin()],
                color,
            });
        }
        lines
    }

    fn initial_code(&self) -> String {
        r#"
// Tutorial 03
// Fly to the target circle and stop. The target is in a random
// location given by the "target" variable.

fn tick() {
    api.accelerate(0.1 * (target - api.position()));
}
"#
        .trim()
        .to_string()
    }

    fn solution(&self) -> String {
        r#"
// Tutorial 03
// Fly to the target circle and stop. The target is in a random
// location given by the "target" variable.

fn tick() {
    let dp = target - api.position();
    if dp.magnitude() < 50.0 {
        api.accelerate(api.velocity() * -10.0);
    } else {
        if api.velocity().magnitude() < 100.0 {
            api.accelerate(dp);
        }
    }
}
"#
        .trim()
        .to_string()
    }
}

struct Tutorial04 {
    target: Point2<f64>,
}

impl Tutorial04 {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let size = 500.0;
        let range = -size..size;
        Self {
            target: point![rng.gen_range(range.clone()), rng.gen_range(range)],
        }
    }
}

impl Scenario for Tutorial04 {
    fn name(&self) -> String {
        "tutorial04".into()
    }

    fn init(&mut self, sim: &mut Simulation) {
        ship::create(sim, 0.0, 0.0, 0.0, 0.0, 0.0, fighter());
        if let Some(&handle) = sim.ships.iter().next() {
            let c = sim.ship_controllers.get_mut(&handle);
            c.unwrap().write_target(self.target.coords);
        }
        ship::create(
            sim,
            self.target.x,
            self.target.y,
            0.0,
            0.0,
            0.0,
            asteroid(1),
        );
    }

    fn status(&self, sim: &Simulation) -> Status {
        if sim.ships.iter().len() > 1 {
            Running
        } else {
            Status::Finished
        }
    }

    fn initial_code(&self) -> String {
        r#"
// Tutorial 04
// Destroy the asteroid. The target is in a random
// location given by the "target" variable.

fn tick() {
    api.accelerate(0.1 * (target - api.position()));
    api.fire_weapon();
}
"#
        .trim()
        .to_string()
    }

    fn solution(&self) -> String {
        r#"
// Tutorial 04
// Destroy the asteroid. The target is in a random
// location given by the "target" variable.

fn turn(speed) {
    let acc = 10.0;
    let margin = 0.01;
    let av = api.angular_velocity();
    if av < speed - margin {
        api.torque(acc);
    } else if av > speed + margin {
        api.torque(-acc);
    }
}

fn normalize_heading(h) {
    while h < 0.0 {
        h += 2 * PI();
    }
    while h > 2 * PI() {
        h -= 2 * PI();
    }
    h
}

fn turn_to(target_heading) {
    let speed = 1.0;
    let margin = 0.1;
    let dh = (api.heading() - target_heading) % (2 * PI());
    if dh - margin > 0.0 {
        turn(-speed);
    } else if dh + margin < 0.0 {
        turn(speed);
    } else {
         turn(-dh);
    }
}

fn tick() {
    turn_to((target - api.position()).angle());
    api.fire_weapon();
}
"#
        .trim()
        .to_string()
    }
}