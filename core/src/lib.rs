//! Core crate of MinecraftRS.
//!
//! This crate defines core structures required by all other crates from MinecraftRS. It is
//! directly inspired by Feather, a Minecraft server written in Rust for latest release.
//! However MinecraftRS aims to replace the static generated blocks and biomes that is used
//! by Feather by runtime palettes. This allows the user of this crate to fully customize
//! the content allowed in its world. The `worldgen` crate provides different features
//! to be able to generate as many different versions as possible, the core crate also
//! provides defaults palettes for blocks and biomes according to the latest release.
//!
//! Feather:
//!   Github:  https://github.com/feather-rs/feather
//!   License: Apache License 2.0
//!
//! MinecraftRS:
//!   Github:  https://github.com/mindstorm38/minecraftrs
//!   Author:  Théo Rozier

pub mod math;
pub mod rand;
pub mod util;
// pub mod phys;

pub mod biome;
pub mod block;
pub mod world;
