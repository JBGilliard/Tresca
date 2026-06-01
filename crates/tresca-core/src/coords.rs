//! Three coordinate types, each a distinct newtype so they can't be
//! accidentally interchanged: WorldPos(IVec3) for global voxel coordinates
//! (signed integers, world can extend in any direction from origin),
//! ChunkPos(IVec3) for which chunk a position lives in, and LocalPos(UVec3)
//! for the 0..32 position within a chunk. Conversions go through explicit
//! constructors: WorldPos::to_chunk() returns (ChunkPos, LocalPos). Arithmetic
//! is defined on WorldPos (add a IVec3 offset, subtract two positions to get a
//! delta). This type discipline catches the off-by-one bugs that plague voxel
//! engines.
