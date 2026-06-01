//! Narrow-phase collision: voxel-aware contact detection. For a pair of dynamic
//! bodies, transform each body's voxels into the other's local frame and test
//! for voxel overlap. For body-vs-static-world, transform the body's voxels
//! into world coordinates and test each against World::get_voxel(). Each
//! voxel-pair contact produces a Contact struct with the contact normal (the
//! face normal of the contact direction), the contact point (the midpoint of
//! the overlapping voxels), and the penetration depth (the overlap distance).
//!
//! For v0.1, no continuous collision detection — fast-moving bodies can tunnel
//! through thin walls. Acceptable at the structural scales we're targeting.
//! v0.5 work adds CCD.
