//! First-person camera controller. update(camera: &mut Camera, input:
//! &InputState, dt: f32). WASD translates the camera in its local frame. Mouse
//! delta rotates the camera (yaw on horizontal, pitch on vertical, clamping
//! pitch to +-89 degrees to prevent gimbal flip). Q/E for vertical movement.
//! Movement speed is configurable; default 5 m/s, with shift for boost.
//!
//! The voxel destruction interaction also lives in app code — probably in
//! main.rs directly for v0.1. When the user clicks, the app constructs a ray
//! from the camera position and the mouse-direction-into-world, then runs
//! Amanatides-Woo voxel traversal against the world. The first non-empty voxel
//! hit is the target. The voxel is set to empty via world.set_voxel(). The
//! world is now dirty in two senses: the chunk needs remeshing (handled by the
//! renderer noticing the dirty flag next frame), and the connectivity may have
//! changed (the solver runs and may spawn dynamic bodies).
