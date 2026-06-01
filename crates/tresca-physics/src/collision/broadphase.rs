//! Broad-phase collision detection. AABB tests over a spatial grid. Hashing
//! bodies by their world-space AABB into a coarse 3D grid (cell size around 4
//! meters, larger than any single chunk). For each body, query the grid for
//! potential collision partners. Output is a list of body pairs to test in
//! narrow phase, plus a list of bodies whose AABB intersects the static world
//! (which need narrow-phase test against static voxels).
