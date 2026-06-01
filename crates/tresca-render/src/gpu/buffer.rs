//! Typed wrappers around wgpu::Buffer. VertexBuffer<T: Pod>, IndexBuffer,
//! UniformBuffer<T: Pod>. Construction takes the data and uploads it; updates
//! can replace the contents (small buffers are mapped and written; large ones
//! use the queue write_buffer). These wrappers exist so the renderer code
//! doesn't have to think about bytemuck::cast_slice or buffer usage flags every
//! time.
