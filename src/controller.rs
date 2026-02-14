use std::fmt::Debug;

use avian2d::prelude::*;
use bevy::prelude::*;

/// A floating character controller, which works by floating the collider off the ground
/// The acceleration applied is independent of the rigid body's mass
/// Note that this does **not** handle movement and input for you - it simply helps with making movement on slopes and stairs smoother
#[derive(Component, Debug, PartialEq)]
#[require(RigidBody)]
pub struct Controller {
    /// The strength of the spring force trying to keep it off the ground. Default value is 512
    pub spring_strength: f32,
    /// The strength of the spring damping - the lower the number, the bouncier the movement feels. Default value is 32
    pub spring_damping: f32,
    /// How much the raycast is allowed to penetrate the ground. Increase if the controller isn't grounded when going down slopes. Default value is 32
    pub ray_penetration: f32,
    /// The target height off the ground, relative to the transform's origin
    pub ride_height: f32,
    /// Set to true to skip the forces getting applied, for example, when jumping
    pub skip_acceleration: bool,
    is_grounded: bool,
}

pub(crate) fn update_controllers(controllers: Query<(&mut Controller, &RayHits, Forces)>) {
    for (mut controller, hits, mut forces) in controllers {
        if hits.is_empty() {
            controller.is_grounded = false;
            continue;
        }

        let hit = hits[0];
        let difference = hit.distance - controller.ride_height;

        controller.is_grounded = true;

        if controller.skip_acceleration {
            continue;
        }

        let ray_vel = Vec2::NEG_Y.dot(forces.linear_velocity());

        let force =
            (difference * controller.spring_strength) - (ray_vel * controller.spring_damping);
        forces.apply_linear_acceleration(Vec2::NEG_Y * force);
    }
}

impl Controller {
    /// Default value for [`spring_strength`](Controller::spring_strength)
    pub const DEFAULT_SPRING_STRENGTH: f32 = 512.0;
    /// Default value for [`spring_damping`](Controller::spring_damping)
    pub const DEFAULT_SPRING_DAMPING: f32 = 32.0;
    /// Default value for [`ray_penetration`](Controller::ray_penetration)
    pub const DEFAULT_RAY_PENETRATION: f32 = 32.0;

    /// Creates a new [`Controller`] with the specified [`ride_height`](Controller::ride_height). Everything else is the defaults
    #[allow(clippy::new_ret_no_self)]
    pub fn new(ride_height: f32) -> Self {
        Self {
            ride_height,
            spring_strength: Self::DEFAULT_SPRING_STRENGTH,
            spring_damping: Self::DEFAULT_SPRING_DAMPING,
            ray_penetration: Self::DEFAULT_RAY_PENETRATION,
            skip_acceleration: false,
            is_grounded: false,
        }
    }

    /// Gets the bundle for the controller
    /// It contains:
    /// - The controller
    /// - A dynamic rigid body
    /// - A lock on rotation
    /// - A raycaster
    pub fn get_bundle(self) -> impl Bundle {
        (
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            RayCaster::new(Vec2::ZERO, Dir2::NEG_Y)
                .with_max_hits(1) // we only care about the closest hit
                .with_max_distance(self.ride_height + self.ray_penetration),
            self,
        )
    }

    /// Is the controller grounded (does the raycast with length [`ride_height`](Controller::ride_height) + [`ray_penetration`](Controller::ray_penetration) collide with anything)?
    pub fn is_grounded(&self) -> bool {
        self.is_grounded
    }

    /// Set the [`spring_strength`](Controller::spring_strength) and return itself
    pub fn with_spring_strength(mut self, value: f32) -> Self {
        self.spring_strength = value;
        self
    }

    /// Set the [`spring_damping`](Controller::spring_damping) and return itself
    pub fn with_spring_damping(mut self, value: f32) -> Self {
        self.spring_damping = value;
        self
    }

    /// Set the [`ray_penetration`](Controller::ray_penetration) and return itself
    pub fn with_ray_penetration(mut self, value: f32) -> Self {
        self.ray_penetration = value;
        self
    }
}
