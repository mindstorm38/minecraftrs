//!
//! Naming convention for generators modules:
//!
//! `"gen" + <major_version> + <minor_version>` with `minor_version` being padded to 2 digits.
//!

#[cfg(feature = "release-1-2")]
pub mod release102;

pub mod legacy;
pub mod biome;

// pub mod r101;
pub mod r102;
// pub mod r103;
// pub mod r104;
// pub mod r105;
// pub mod r106;
// pub mod r107;
// pub mod r108;
// pub mod r109;
// pub mod r110;
// pub mod r111;
// pub mod r112;
