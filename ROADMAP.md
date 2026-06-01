# Tresca Roadmap

This document is provisional. Priorities shift with what's learned during development.

## v0.1 — Foundation (in progress)

- wgpu renderer with textured voxel chunks
- Dense 32³ chunk storage
- Naive cube-face meshing
- WASD camera, basic input
- Click-to-destroy single voxels
- Single-material world

## v0.2 — Multi-material destruction

- Material table with continuum mechanics parameters
- Material-specific destruction patterns (drywall crumbles, wood splinters, concrete chunks)
- Connected component analysis for chunk separation
- Basic rigid body physics for separated chunks

## v0.3 — Structural mechanics

- Connectivity graph over voxel chunks
- Load path propagation under gravity
- Tresca and von Mises yield criteria
- Progressive collapse from local failure
- Anchor system (ground, scripted anchors)

## v0.4 — Performance + scale

- Greedy meshing
- GPU compute for voxel updates
- Frustum and occlusion culling
- Larger worlds (5-10M active voxels)

## v0.5 — Forces and effects

- Explosion physics with blast propagation
- Fragmentation modeling
- Projectile and impact systems
- Damage decals and persistent visual state

## v0.6 — Soft body

- Position-based dynamics for cloth, ropes
- Limited deformable solid support
- Material point method evaluation for soft body

## v1.0 — Production readiness

- Stable public API
- Authoring tools (basic scene editor)
- Save/load format
- Documentation completeness
- Performance benchmarks
- Example scenarios
