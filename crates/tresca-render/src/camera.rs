//! Camera struct: position (Vec3), orientation (Quat), field of view, aspect
//! ratio, near/far planes. Methods: view_matrix(), projection_matrix(),
//! view_projection(). Also the uniform-buffer-compatible struct CameraUniform
//! { view_proj: [[f32; 4]; 4], world_pos: [f32; 4] } (the world_pos uses vec4
//! for alignment) and a method to build the uniform from the camera. No input
//! handling — this is data only. The camera controller in tresca-app mutates
//! Camera directly.
