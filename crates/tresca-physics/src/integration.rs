//! Semi-implicit Euler integration at fixed timestep. integrate(body: &mut
//! RigidBody, dt: f32):
//!
//! 1. Apply gravity to accumulated force.
//! 2. Update linear velocity: v += (force / mass) * dt.
//! 3. Update position: p += v * dt.
//! 4. Update angular velocity: ω += (I_inv * τ) * dt, where I_inv is in world
//!    coordinates (rotate the local inverse inertia tensor by the body's
//!    orientation).
//! 5. Update orientation: q += 0.5 * Quat::from_axis_angle(ω.normalize(),
//!    ω.length() * dt) * q. Then normalize the quaternion to prevent drift.
//! 6. Clear accumulated force and torque.
//!
//! Fixed timestep is 1/60 second. Variable framerate is handled by the app
//! accumulator pattern (integrate until accumulated time is less than one
//! timestep).
