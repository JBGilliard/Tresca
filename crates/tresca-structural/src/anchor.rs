//! Anchor identification. For v0.1, anchors are determined by a single rule:
//! any voxel with y == 0 (sitting on the ground plane) is anchored. Any voxel
//! face-connected to an anchored voxel is transitively anchored. Future
//! versions allow explicit anchor flags on voxels (the ANCHORED flag from
//! voxel.rs) for scripted scene structure.
