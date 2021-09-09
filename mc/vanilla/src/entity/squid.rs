use mc_core::entity::{EntityComponent, EntityCodec, DefaultEntityCodec, SingleEntityCodec};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;
use std::convert::TryFrom;
use mc_core::util::NbtExt;

#[derive(Debug, Default)]
pub struct SquidEntity;
impl EntityComponent for SquidEntity {
    const CODEC: &'static dyn EntityCodec = &DefaultEntityCodec::<SquidEntity>::new();
}

#[derive(Debug, Default)]
pub struct GlowSquidEntity {
    dark_ticks_remaining: u32
}

impl EntityComponent for GlowSquidEntity {
    const CODEC: &'static dyn EntityCodec = &GlowSquidEntityCodec;
}

pub struct GlowSquidEntityCodec;
impl SingleEntityCodec for GlowSquidEntityCodec {

    type Comp = GlowSquidEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i32("DarkTicksRemaining", src.dark_ticks_remaining as i32);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        GlowSquidEntity {
            dark_ticks_remaining: src.get_i32_or("DarkTicksRemaining", 0) as u32
        }
    }

}
