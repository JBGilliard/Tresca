//! The Voxel struct itself, the eight-byte packed layout from ARCHITECTURE.md.
//! Internally a u64, with bit-masked accessors for material ID (16 bits),
//! damage state (8 bits), flags (8 bits), density override (16 bits), and
//! reserved (16 bits). Methods include Voxel::empty() (returns a sentinel
//! value, the all-zero voxel), Voxel::is_empty(), Voxel::material(),
//! Voxel::with_material(), Voxel::set_flag(), and so on. All accessors are
//! const fn where possible because they're called millions of times per frame.
//! The crate provides bytemuck::Pod and bytemuck::Zeroable impls so voxels can
//! be uploaded directly to GPU buffers. For v0.1, only material ID, damage
//! state, and one flag (the ANCHORED flag) actually do anything; the density
//! override and reserved bits are read and stored but no code consumes them yet.
