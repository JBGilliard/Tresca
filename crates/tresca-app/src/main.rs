//! Entry point. Initializes tracing for structured logging. Creates the winit
//! event loop. Creates the window. Constructs the Device (from
//! tresca-render::gpu). Loads materials from assets/materials.toml. Constructs
//! the World from assets/demo_scene.toml. Constructs the Renderer,
//! PhysicsWorld, ConnectivitySolver, Camera, CameraController, InputState.
//! Enters the event loop:
//!
//! - On window event: update input state, handle resize.
//! - On about-to-wait: poll input, update camera via controller, handle any
//!   pending user actions (mouse clicks -> ray cast -> voxel destruction). If
//!   the world was modified, run the connectivity solver. Integrate physics
//!   with the accumulator pattern. Request a redraw.
//! - On redraw: call renderer.render(&world, &physics, &camera).
//!
//! About 200 lines for v0.1.

fn main() {}
