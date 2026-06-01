//! The RigidBody struct: BodyId, Vec<Voxel> (the voxel content, in local body
//! coordinates), Vec3 (center of mass in local coords), f32 (mass), f32
//! (inverse mass, cached), Mat3 (inertia tensor in local coords), Mat3 (inverse
//! inertia tensor, cached), Vec3 (world position), Quat (world orientation),
//! Vec3 (linear velocity), Vec3 (angular velocity), Vec3 (accumulated force),
//! Vec3 (accumulated torque).
//!
//! The crucial constructor is RigidBody::from_voxels(voxels: &[(LocalPos,
//! Voxel)], materials: &MaterialRegistry, world_position: Vec3) -> RigidBody.
//! This computes mass properties: total mass is the sum of (voxel volume ×
//! material density) for each voxel, center of mass is the mass-weighted
//! average position, the inertia tensor is computed by summing each voxel's
//! contribution (treat each voxel as a small cube and use the standard cube
//! inertia formula plus the parallel axis theorem). The body's local coordinate
//! origin is placed at the computed center of mass. Cached inverse mass and
//! inverse inertia tensor are computed once at construction.
