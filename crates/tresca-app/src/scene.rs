//! Scene loading from TOML. The demo scene file lists chunk positions and what
//! voxel content to fill them with — for v0.1, either by enumerating voxel
//! positions explicitly or by primitive shapes ("solid box from (x1,y1,z1) to
//! (x2,y2,z2) of material concrete"). Parses, validates against the material
//! registry, populates the World.
