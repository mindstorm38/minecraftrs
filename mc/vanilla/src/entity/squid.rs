use mc_core::entity::{EntityComponent, EntityCodec};
use mc_core::default_entity_codec;

#[derive(Debug, Default)]
pub struct SquidEntity;
impl EntityComponent for SquidEntity {
    const CODEC: &'static EntityCodec = &default_entity_codec!(SquidEntity);
}

#[derive(Debug, Default)]
pub struct GlowSquidEntity {
    dark_ticks_remaining: u32
}

