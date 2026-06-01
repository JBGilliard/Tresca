//! The naive cube-face meshing algorithm. Single public function:
//! pub fn mesh_chunk(chunk: &Chunk, world: &World, chunk_pos: ChunkPos)
//! -> (Vec<Vertex>, Vec<u32>). The implementation iterates every voxel in the
//! chunk. For each non-empty voxel, for each of its six faces (±X, ±Y, ±Z),
//! it queries whether the neighbor is empty. If the neighbor is within this
//! chunk, the lookup is a direct chunk index. If the neighbor is outside the
//! chunk, the lookup goes through world.get_voxel() to check the adjacent
//! chunk. If the neighbor is empty, the algorithm emits four vertices and six
//! indices (two triangles) for that face, with positions in chunk-local
//! coordinates, the correct face normal, and the material ID of the source
//! voxel.
//!
//! The output buffers are pre-sized roughly (32³ * 6 faces is the absolute
//! upper bound, but a typical chunk has maybe 10% of that). The function takes
//! 1–3 ms per chunk on a single CPU core at 32³ — fine for v0.1 since only
//! dirty chunks remesh per frame and at most a handful are dirty in any given
//! frame.
