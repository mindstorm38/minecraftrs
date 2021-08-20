//! Core crate of MinecraftRS.
//!
//! This crate defines core structures required by all other crates from MinecraftRS. It is
//! directly inspired by Feather, a Minecraft server written in Rust for latest release.
//! However MinecraftRS aims to replace the static generated blocks and biomes that is used
//! by Feather by runtime palettes. This allows the user of this crate to fully customize
//! the content allowed in its world.
//!
//! The goal of this crate is to provide structure that statically describes the world, in
//! a frozen way. It provides tools to make levels and entities running and living. This
//! crate also provides default level sources such as anvil level loader or super flat
//! level generator.
//!
//! If you want real world generation for a specific Minecraft version, check out the
//! `mc-worldgen` crate, and if you want to run a world, check `mc-runtime`.
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
pub mod pos;

pub mod biome;
pub mod block;
pub mod item;
pub mod world;
