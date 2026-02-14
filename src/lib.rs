//! A basic 2D floating character controller, implemented using the [Avian](https://docs.rs/avian2d/latest/avian2d) physics engine for the [Bevy](https://bevyengine.org) game engine
//! The controller works by floating the collider off the ground slightly, to avoid issues with friction, slopes, stairs etc. (like [Very Very Valet's character controller](https://www.youtube.com/watch?v=qdskE8PJy6Q))
//!
//! Note that this crate does **not** handle any input or movement for you, it just helps with floating the collider.
//! If you're looking for an out-of-the-box solution, check out [Tnua](https://docs.rs/bevy-tnua/latest/bevy_tnua), which uses a similar principle
//!
//! To get started, look at the [`Controller`](controller::Controller) component

mod controller;
mod plugin;

pub use controller::Controller;
pub use plugin::ControllerPlugin;
