# Tresca Architecture

## Overview

Tresca is a real-time multi-material destruction physics engine. The novel claim is structural integrity: rather than modeling destruction as material ablation (Teardown's approach) or pre-baked fracture (mesh-based approaches), Tresca models the load path through a structure and predicts failure when stresses exceed material yield criteria. Collapse emerges from local failures propagating through the structure under gravity.

The design optimizes for three properties, in order:

1. Physical correctness at the scale of structural elements
2. Real-time performance on consumer GPUs
3. Architectural clarity sufficient that the engine can be read and modified by a single developer

## Crates/Layers

The engine is organized as four crates, each with a well-defined responsibility:

**tresca-core** defines foundational types: voxel grids, chunks, material definitions, and the world model.

**tresca-render** is the wgpu-based renderer. Consumes the world model and produces frames. Responsible for chunk meshing, GPU resource management, lighting, and visualization of physical quantities (stress fields, fracture surfaces) as debug overlays.

**tresca-physics** handles classical rigid body dynamics for chunks that have separated from the structure. When a section disconnects from any structural anchor, it becomes a dynamic rigid body subject to gravity, contact, and friction. This crate also handles applied forces (impacts, explosions) and translates them into stress fields the structural solver consumes.

**tresca-structural** is the differentiator. It models the load path through connected voxels, computes stress fields under applied forces, predicts failure via material yield criteria, and identifies which connections have broken in a given step. The output is a set of disconnected components, which tresca-physics then handles as rigid bodies.

The application crate **tresca-app** ties these together: window, input loop, scene loader, and the render tick that advances physics, queries the structural solver, and submits a frame.

## Voxel representation

The world is a sparse set of chunks. Each chunk is a 32×32×32 dense array of voxels. Empty space is implicit (no chunk allocated). This balances per-voxel memory cost against locality: dense within a chunk for cache-friendly access during meshing and physics, sparse across chunks for unbounded worlds.

Each voxel occupies 8 bytes:

- Material ID (2 bytes): index into a material table
- Damage state (1 byte): 0-255 accumulated stress/wear
- Flags (1 byte): special properties — load-bearing marker, structural anchor, sensor
- Density override (2 bytes): per-voxel density variation for irregular materials
- Reserved (2 bytes): future use

Material properties (Young's modulus, yield stress, density, fracture toughness, etc.) live in a separate material table, indexed by Material ID. This avoids storing 50+ bytes per voxel and allows hot-swapping material definitions at runtime.

## Material model

Each material is defined by continuum mechanics parameters:

- **Density** (ρ) — mass and gravitational load
- **Young's modulus** (E) — elastic deformation
- **Poisson's ratio** (ν) — lateral strain coupling
- **Yield stress** (σ_y) — threshold for permanent deformation
- **Ultimate tensile strength** (σ_u) — fracture threshold
- **Fracture toughness** (K_IC) — crack propagation resistance
- **Failure mode** — brittle, ductile, or granular

Materials are loaded from TOML files in the world definition. A default material set (concrete, steel, wood, drywall, glass) ships with the engine. User materials can be registered at world-load time.

## Rendering pipeline

The renderer uses wgpu directly with WGSL shaders. The pipeline:

1. **Chunk meshing**: when a chunk's voxel state changes, the renderer extracts a triangle mesh for that chunk. v0.1 uses naive cube faces (one quad per exposed voxel face). Greedy meshing and surface nets are planned for performance.
2. **Material binding**: each material has a corresponding shader uniform set containing its texture, color, surface properties, and rendering-relevant parameters.
3. **Forward rendering with deferred decals**: the main scene renders forward; damage and scorch decals apply in a separate pass. 
4. **Debug visualization layer**: stress fields, load paths, fracture surfaces, and physics debug data render as semi-transparent overlays controlled by feature flags.
5. **Post-processing**: HDR tonemapping, optional bloom, color grading.

Compute shaders handle parallel work: voxel state updates from physics, mesh extraction, and the structural mechanics solver. Heavy use of WGSL compute pipelines is core to meeting performance targets.

## Physics integration

Rigid body physics handles two object kinds:

**Static structure**: voxel chunks still connected to a structural anchor. Static objects do not move under gravity; they hold each other up. The structural solver decides when a chunk transitions from static to dynamic.
**Dynamic chunks**: voxel chunks that have separated from any anchor. Subject to gravity, contact, friction, and applied forces. Integration is semi-implicit Euler at fixed timestep with continuous collision detection against the remaining static structure and other dynamic chunks.

Soft body deformation is out of scope for v0.1. Future versions will add position-based dynamics for cloth, ropes, and constrained soft body behavior.

## Structural mechanics solver

The core technical contribution. Each update:

1. **Build the connectivity graph**: voxels in the static structure are grouped into nodes. Two voxels share a node if they share a face and are the same material (or bonded compatible materials). Edges in the graph are connections between nodes — face contacts across material boundaries, with edge capacity determined by the weaker of the two materials' shear and tensile strengths.
2. **Identify anchors**: nodes touching the world's structural anchor set (typically the ground plane, or scripted anchor voxels) are graph roots.
3. **Propagate loads**: from each non-anchor node, distribute gravitational load to neighbors that lead toward an anchor. Distribution follows a least-resistance path weighted by edge capacity. Applied forces from physics (impacts, explosions) add to the local stress field.
4. **Check yield**: each edge is evaluated against the Tresca yield criterion (or von Mises in materials configured for ductile failure). Edges exceeding yield stress are marked failed.
5. **Recompute connectivity**: failed edges break their nodes apart. Connected component analysis identifies sub-structures that have lost connection to any anchor.
6. **Promote to dynamic**: disconnected sub-structures are handed to the physics layer as rigid bodies with appropriate mass, center of mass, and inertia tensor derived from voxel contents.

The solver runs on the GPU in compute shaders, with the connectivity graph stored as compact GPU buffers. Structural integrity is rechecked when voxels are modified, when forces exceed a threshold, or on a periodic tick — not every frame.

## Performance targets

60 FPS at 1080p with a structure of 1-2 million active voxels on a consumer GPU (RTX 3060-class or equivalent).

Key levers:

- Chunk granularity (32³ default; tunable)
- Meshing strategy (naive → greedy → surface nets, in increasing complexity)
- Structural update frequency (event-driven, not per-frame)
- GPU compute for parallel work
- Frustum and occlusion culling on chunks

## Development principles

**Physical correctness over visual flourish.** When in doubt, prefer the answer that matches what would actually happen to a real structure. Visual polish is layered on top of a correct simulation, never used to hide an incorrect one.
**Reproducibility.** Simulations are deterministic given the same inputs. Seeded RNG, fixed timestep physics, no hidden GPU-driven nondeterminism. Critical for both debugging and research applications.
**Pure Rust except where physics demands otherwise.** wgpu and WGSL are the only non-Rust dependencies. No C/C++ FFI for the engine core.

## Open questions

Several major design decisions are explicitly deferred:

- Meshing strategy at very high voxel counts (surface nets vs. dual contouring vs. GPU-raymarched rendering)
- Soft body integration scheme (position-based dynamics vs. material point method)
- Networking and multi-user simulation
- Authoring tools and scene editing
- Save/load format

These will be addressed in subsequent versions.
