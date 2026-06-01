//! Contact resolution between rigid bodies and between a rigid body and the
//! static world. A Contact struct holds two body references (or body + world
//! flag), contact point in world coordinates, contact normal, penetration
//! depth. Resolution applies an impulse to separate the bodies along the
//! contact normal, sized to remove relative velocity at the contact point plus
//! a small restitution and a positional correction term to handle Baumgarte
//! stabilization. For v0.1, friction is approximated as a fixed coefficient
//! applied tangentially. No rolling friction.
//!
//! This is the most algorithm-heavy file in the crate. It's where you implement
//! the impulse-based contact resolver from Ericson's Real-Time Collision
//! Detection or Millington's Game Physics Engine Development. Roughly 300 lines
//! for v0.1.
