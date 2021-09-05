//! Codec basics for ECS entities' components.

use hecs::{EntityRef, EntityBuilder};
use nbt::CompoundTag;


/// This structure describes a specific way of encoding, decoding and building a default variant
/// of a component structure for an entity. This structure can be and must be defined statically
/// because most functions take a static lifetime reference.
///
/// Components that can be encoded and decoded are called "data components", in opposition to
/// "runtime components" that are not saved when saving the world and are just
pub struct EntityCodec {
    /// A function called to encode an entity's component into a NBT compound tag.
    /// You should get the component from the given `EntityRef`.
    pub encode: fn(src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String>,
    /// A function called to decode an entity's component from a NBT compound tag.
    /// If the tag does not provide required values, put default ones, and then add
    /// the component to the given `EntityBuilder`, if  the missing tags are critical,
    /// return an `Err` with a description of the error.
    pub decode: fn(src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String>,
    /// Create a default variant of this entity component.
    pub default: fn(dst: &mut EntityBuilder),
}

impl EntityCodec {

    pub fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        (self.encode)(src, dst)
    }

    pub fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        (self.decode)(src, dst)
    }

}

#[allow(unused_variables)]
pub fn encode_noop(src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> { Ok(()) }

#[allow(unused_variables)]
pub fn decode_noop(src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> { Ok(()) }


#[macro_export]
macro_rules! default_entity_codec {
    ($type_with_default:ty) => {
        $crate::entity::EntityCodec {
            encode: $crate::entity::encode_noop,
            decode: $crate::entity::decode_noop,
            default: |dst| {
                dst.add(<$type_with_default as Default>::default());
            }
        }
    };
}
