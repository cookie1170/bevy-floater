# bevy_floater
A physics-based 2D floating character controller for the [Bevy](https://bevyengine.org) game engine using the [Avian](https://docs.rs/avian2d/latest/avian2d) physics engine
It works by floating the collider off the ground. This helps with sticking to slopes, ground clearance, avoiding friction etc.

Note that this controller does **not** handle movement or input for you. It just helps with floating the collider off the ground
If you want a controller that works out-of-the-box, then check out [Tnua](https://docs.rs/bevy_tnua/latest/bevy_tnua), which works using a similar principle

The implementation is inspired by [Very Very Valet's character controller](https://www.youtube.com/watch?v=qdskE8PJy6Q)

