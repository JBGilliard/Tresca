//! The World type, a HashMap<ChunkPos, Box<Chunk>> plus the material registry.
//! The chunks are boxed because they're large (256 KiB each) and we don't want
//! them living inline in the hash map's bucket array. Public methods are
//! coordinate-agnostic: get_voxel(world_pos: WorldPos) -> Voxel,
//! set_voxel(world_pos: WorldPos, voxel: Voxel), iter_dirty_chunks(),
//! clear_dirty_flags(), dimensions(). Internally set_voxel decomposes the world
//! position into chunk coordinates and local coordinates, allocates the chunk
//! if it doesn't exist, modifies the voxel, and marks the chunk dirty. For
//! chunk-boundary writes, neighboring chunks are also marked dirty because the
//! meshing of those chunks may have referred to this voxel as a neighbor.
//!
//! The World also owns the anchor set — a list of voxel positions or chunk
//! positions that are considered structurally fixed (the ground plane for v0.1,
//! but extensible to scripted anchors later). The connectivity solver in
//! tresca-structural queries this set.
