# Tresca

**Real-time destruction physics with proper structural mechanics.**

Tresca simulates structural failure from first principles. Unlike voxel-destruction systems (*Teardown*) that treat material as uniform mass to be ablated, Tresca models each chunk's material properties, computes load paths through the structure under gravity and applied forces, and predicts failure using yield criteria from continuum mechanics.

Named for Henri Tresca, 19th-century French mechanical engineer who developed the Tresca yield criterion, the mathematical condition that predicts when a material will fail under stress.

## Use Cases

Tresca is built as a serious computational mechanics + real-time graphics engine, not a game per say. Intended applications:

- Architectural failure analysis and visualization
- Controlled demolition planning
- Structural integrity research and education
- Disaster response and damage assessment training
- A foundation for higher-level simulation work in defense, civil engineering, and computational graphics research

## Building

Requires Rust 1.96 or later and a GPU supporting Vulkan, Metal, or DirectX 12.

```bash
git clone https://github.com/jbgilliard/tresca
cd tresca
cargo run --release
```

## License

Tresca is licensed under Apache 2.0. See [LICENSE](LICENSE).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). For security issues, see [SECURITY.md](SECURITY.md).
