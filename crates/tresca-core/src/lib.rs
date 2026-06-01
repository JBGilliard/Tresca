//! Module declarations and the curated public API surface. Re-exports the
//! canonical names (Voxel, Chunk, World, Material, MaterialId, the coordinate
//! types) so downstream crates can use `tresca_core::{Voxel, World}` without
//! reaching into module paths. Also re-exports Result and Error types from
//! error.rs. The library has zero use statements in its public API that point
//! at internal module paths.
