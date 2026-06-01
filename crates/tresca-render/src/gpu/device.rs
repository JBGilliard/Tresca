//! wgpu device setup. Owns the wgpu::Instance, wgpu::Adapter, wgpu::Device,
//! wgpu::Queue, and the wgpu::Surface. Methods: new(window: &Window) ->
//! Result<Device>, configure_surface(width, height), current_frame() ->
//! SurfaceTexture. Centralizes all the boilerplate for wgpu initialization. On
//! resize, the surface is reconfigured here. The depth texture is also owned
//! here and recreated on resize.
