//! Voxel connectivity tracking. The ConnectivityGraph is a simple
//! representation: for v0.1, just a function
//! compute_connected_components(world: &World) -> Vec<ConnectedComponent>,
//! where each component is a Vec<WorldPos>. The algorithm is BFS over
//! face-adjacent voxels. For v0.3 this becomes a proper graph with stress
//! capacity on edges.
