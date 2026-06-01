//! The Chunk type: a 32×32×32 dense array of voxels stored as
//! [Voxel; 32 * 32 * 32]. Provides indexing by local position (x, y, z each
//! 0..32), conversion of 3D indices to the flat array index
//! (x + y * 32 + z * 32 * 32), iteration helpers (visit all voxels, visit only
//! non-empty voxels, visit face-adjacent neighbors of a given voxel). Tracks a
//! dirty flag set when any voxel changes since the last mesh extraction. The
//! chunk also tracks a generation counter that increments on every modification
//! — the renderer uses this to detect whether its cached mesh is still valid.
//! The chunk does not know its own world position; that lives in the World map.
