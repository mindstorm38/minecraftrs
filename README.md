# Minecraft RS

Pure Rust crates for Minecraft data manipulation. This cargo workspace aims to separate to 
split data structures from actual implementations. The two main crates in this repository
are [core](mc/core) and [vanilla](mc/vanilla), they are the only available for now on
[crates.io](https://crates.io).

Other crate such as [worldgen](mc/worldgen), [runtime](mc/runtime) and [server](mc/server)
are not actively maintained because the effort to keep these up-to-dates to the latest 
versions is too big. The worldgen crate is a little more maintained, but it is still a 
toy crate which currently only implements the 1.2.5 (without generated structures).

## Core crate
The [core](mc/core) crate define the most important and mandatory data structures used by
Minecraft, this includes definition of optimized structures for blocks, biomes, heightmaps, 
entities. The main goal of these tiny structures is to be statically defined, so they don't 
take time to load at runtime, this also allows really simple equality test using their 
pointers. They also have a textual identifier in the usual Minecraft format `namespace:key`.
Once statically defined, they need to be added a runtime registry that associate
their static pointer to a linear positive integer id that can be used to efficiently 
serialize and deserialize these structures **internally at runtime**, for the serialization
in static files, the integer id is mapped to the textual identifier, it's needed because
integer ids might change between different runs of the crate. 

These global registries are bundled in a "level environment" structure that is used by chunks
stored inside a level. This crate also provides a way to source chunks for a level, and 
predefine an anvil chunk source to load world saved by Minecraft in the anvil format.

## Vanilla crate
The [vanilla](mc/vanilla) provides static instances definition to use with global registries,
it also provides a trait `WithVanilla` that is implemented on each structure that supports a
vanilla variants, such as blocks, biomes and level environment.
