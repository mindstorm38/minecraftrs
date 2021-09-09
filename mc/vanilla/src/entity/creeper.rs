use mc_core::entity::SingleEntityCodec;
use mc_core::nbt::CompoundTag;
use mc_core::util::NbtExt;
use mc_core::entity_component;

#[derive(Debug, Default)]
pub struct CreeperEntity {
    /// The radius of the explosion itself, default 3.
    explosion_radius: u8,
    /// States the initial value of the creeper's internal fuse timer (does not affect creepers
    /// that fall and explode upon impacting their victim). The internal fuse timer returns to
    /// this value if the creeper is no longer within attack range. Default 30.
    fuse: u16,
    /// Whether the creeper has been ignited by flint and steel.
    ignited: bool,
    /// True if the creeper is charged from being struck by lightning.
    powered: bool
}

entity_component!(CreeperEntity: CreeperEntityCodec);

pub struct CreeperEntityCodec;
impl SingleEntityCodec for CreeperEntityCodec {

    type Comp = CreeperEntity;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag) {
        dst.insert_i8("ExplosionRadius", src.explosion_radius as i8);
        dst.insert_i16("Fuse", src.fuse as i16);
        dst.insert_bool("ignited", src.ignited);
        dst.insert_bool("powered", src.powered);
    }

    fn decode(&self, src: &CompoundTag) -> Self::Comp {
        CreeperEntity {
            explosion_radius: src.get_i8_or("ExplosionRadius", 0) as u8,
            fuse: src.get_i16_or("Fuse", 0) as u16,
            ignited: src.get_bool_or("ignited", false),
            powered: src.get_bool_or("powered", false)
        }
    }

}
