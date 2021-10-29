use mc_core::entity::{EntityComponent, EntityCodec, DefaultEntityCodec};

#[derive(Debug, Default)]
pub struct CaveSpiderEntity;

impl EntityComponent for CaveSpiderEntity {
    const CODEC: &'static dyn EntityCodec = &DefaultEntityCodec::<CaveSpiderEntity>::new();
}
