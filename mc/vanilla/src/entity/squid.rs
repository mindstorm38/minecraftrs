use mc_core::entity::SingleEntityCodec;
use mc_core::nbt::CompoundTag;
use mc_core::util::NbtExt;
use mc_core::entity_component;

#[derive(Debug, Default)]
pub struct SquidEntity;
entity_component!(SquidEntity: default);

#[derive(Debug, Default)]
pub struct GlowSquidEntity {
    dark_ticks_remaining: u32
}

entity_component!(GlowSquidEntity: GlowSquidEntityCodec);

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
