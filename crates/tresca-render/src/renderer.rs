//! The Renderer type. Owns the wgpu Device and Queue (via the gpu module), the
//! depth texture, the surface, the camera uniform buffer, the material uniform
//! buffer, the chunk mesh cache, and the render pipelines. The single public
//! entry point is render(world: &World, physics: &PhysicsWorld, camera: &Camera)
//! -> Result<()>. Internally that does:
//!
//! 1. Update camera uniform with current view/projection matrices.
//! 2. Walk dirty chunks from world.iter_dirty_chunks(). For each one, call
//!    mesh::naive::mesh_chunk(chunk, world) to produce a fresh vertex/index
//!    buffer. Upload to GPU. Cache by chunk position.
//! 3. Walk dynamic bodies from physics.iter_bodies(). For each one, the body
//!    owns a voxel set; mesh it (cached by body ID until destroyed). Compute
//!    the body's model matrix from position and orientation.
//! 4. Begin a render pass. Bind the forward pipeline. For each chunk's mesh,
//!    bind the material uniforms, set the model matrix (identity for static
//!    chunks), draw indexed. For each dynamic body's mesh, bind its model
//!    matrix, draw indexed.
//! 5. Submit command buffer. Present.
//!
//! Re-meshing happens synchronously on the CPU for v0.1. v0.4 work moves this
//! to a compute shader for parallelism, but for v0.1 the simple CPU path is
//! fast enough at the small scale.
