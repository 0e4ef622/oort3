use super::vec2::Vec2;
use crate::simulation::ship::{ShipAccessor, ShipHandle};
use crate::simulation::Simulation;
use nalgebra::{vector, Point2};
use rhai::plugin::*;

const MAX_RADAR_DISTANCE: f64 = 3000.0;

#[export_module]
pub mod plugin {
    #[derive(Copy, Clone)]
    pub struct RadarApi {
        pub handle: ShipHandle,
        pub sim: *mut Simulation,
    }

    impl RadarApi {
        #[allow(clippy::mut_from_ref)]
        fn sim(&self) -> &mut Simulation {
            unsafe { &mut *self.sim }
        }

        fn ship(&self) -> ShipAccessor {
            self.sim().ship(self.handle)
        }
    }

    pub fn scan(obj: RadarApi) -> ScanResult {
        let sim = obj.sim();
        let own_team = obj.ship().data().team;
        let own_position: Point2<f64> = obj.ship().position().vector.into();
        let mut result = ScanResult {
            found: false,
            position: vector![0.0, 0.0],
            velocity: vector![0.0, 0.0],
        };
        let mut best_distance = 0.0;
        for &other in sim.ships.iter() {
            if sim.ship(other).data().team == own_team {
                continue;
            }
            let other_position: Point2<f64> = sim.ship(other).position().vector.into();
            let distance = nalgebra::distance(&own_position, &other_position);
            if distance > MAX_RADAR_DISTANCE {
                continue;
            }
            if !result.found || distance < best_distance {
                result = ScanResult {
                    found: true,
                    position: other_position.coords,
                    velocity: sim.ship(other).velocity(),
                };
                best_distance = distance;
            }
        }
        result
    }

    #[derive(Copy, Clone)]
    pub struct ScanResult {
        pub found: bool,
        pub position: Vec2,
        pub velocity: Vec2,
    }

    #[rhai_fn(get = "found", pure)]
    pub fn get_found(obj: &mut ScanResult) -> bool {
        obj.found
    }

    #[rhai_fn(get = "position", pure)]
    pub fn get_position(obj: &mut ScanResult) -> Vec2 {
        obj.position
    }

    #[rhai_fn(get = "velocity", pure)]
    pub fn get_velocity(obj: &mut ScanResult) -> Vec2 {
        obj.velocity
    }
}