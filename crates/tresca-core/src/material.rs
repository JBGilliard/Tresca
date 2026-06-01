//! Material struct: name, albedo color, roughness, metallic, density, Young's
//! modulus, Poisson's ratio, yield stress, ultimate tensile strength, fracture
//! toughness, failure mode enum. The rendering-relevant fields (albedo,
//! roughness, metallic) are exercised in v0.1. The mechanics fields are
//! present, serialized from TOML, but the structural solver doesn't read them
//! yet — v0.3 work. MaterialId is a NonZeroU16 newtype so material 0 can be
//! sentinel-empty. MaterialRegistry holds the Vec<Material> and provides
//! register(material) -> MaterialId, get(id) -> &Material, and TOML loading.
//! For v0.1 the registry is populated at startup from assets/materials.toml.
