//! The forward render pipeline. Constructs the wgpu::RenderPipeline for voxel
//! rendering: takes the voxel.wgsl shader, the vertex layout (position, normal,
//! material_id), the bind group layouts (camera uniform, material uniforms),
//! enables depth testing, sets cull mode to back-face. Single public type
//! ForwardPipeline with new(device, format) and bind(render_pass).
