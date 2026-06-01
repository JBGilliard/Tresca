// The vertex+fragment shader for voxel rendering. Vertex shader: transforms
// world-space vertex position by the model-view-projection matrix, passes
// through normal and material ID to the fragment shader. Fragment shader: looks
// up material parameters from a uniform array indexed by material ID, computes
// simple diffuse lighting with one directional light (Lambert), adds an ambient
// term, multiplies by material albedo. Outputs final color. No PBR yet — that's
// v0.3 work. The shader is maybe 60 lines.
