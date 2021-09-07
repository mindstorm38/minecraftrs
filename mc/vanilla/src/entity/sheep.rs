use crate::util::DyeColor;
use mc_core::entity::{EntityCodec, EntityComponent};
use mc_core::hecs::{EntityRef, EntityBuilder};
use mc_core::nbt::CompoundTag;

#[derive(Debug, Default)]
pub struct SheepEntity {
    /// The color of the sheep
    color: DyeColor,
    /// True if the sheep has been shorn.
    sheared: bool
}

impl EntityComponent for SheepEntity {
    const CODEC: &'static dyn EntityCodec = &SheepEntityCodec;
}

pub struct SheepEntityCodec;
impl EntityCodec for SheepEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<SheepEntity>() {
            dst.insert_i8("Color", comp.color.get_id() as i8);
            dst.insert_bool("Sheared", comp.sheared);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(SheepEntity {
            color: DyeColor::from_id(src.get_i8("Color").unwrap_or_default() as u8),
            sheared: src.get_bool("Color").unwrap_or_default()
        });
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(SheepEntity::default());
    }

}
