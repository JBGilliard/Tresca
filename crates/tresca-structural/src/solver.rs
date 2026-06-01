//! The ConnectivitySolver type. Public entry:
//! check_connectivity(world: &World, physics: &mut PhysicsWorld) -> Result<()>.
//! Implementation:
//!
//! 1. Run anchor identification: which voxels are anchored?
//! 2. Find connected components of non-anchored voxels.
//! 3. For each connected component, extract its voxels from the world (set them
//!    to empty in the world), construct a RigidBody from them via
//!    RigidBody::from_voxels, and add it to physics.
//!
//! This runs only when the world has been modified (the solver tracks a
//! generation counter against the world). Per-frame cost when nothing changed
//! is essentially zero.
