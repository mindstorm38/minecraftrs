//! Codec basics for ECS entities' components.

use std::marker::PhantomData;

use hecs::{EntityRef, EntityBuilder, Component};
use nbt::CompoundTag;


/*/// This structure describes a specific way of encoding, decoding and building a default variant
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
}*/


/// This trait describes a specific way of encoding, decoding and building a default variant
/// of a component structure for an entity. This trait should usually be implemented for each
/// component of an entity, however because it doesn't provide any type restriction you can
/// encode, decode and add whatever default value you want.
///
/// Structures implementing this trait can be zero-sized and defined statically, doing this
/// allows you to make `&'static dyn EntityCodec` references that can be used as constants to
/// define default codecs for an entity component structure.
pub trait EntityCodec: Send + Sync {

    /// Encode components stored accessible from the given entity reference into given destination
    /// compound tag.
    #[allow(unused_variables)]
    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        Ok(())
    }

    /// Decode given source compound tag and add decoded components into the given entity builder.
    #[allow(unused_variables)]
    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        Ok(())
    }

    /// Add default components to the given entity builder.
    #[allow(unused_variables)]
    fn default(&self, dst: &mut EntityBuilder);

}


/// A useful structure that implements the method `EntityCodec::default` and add to the given
/// builder the `Default:default()` value of the generic type `T`. Actually, `EntityCodec` is
/// only implemented when your type `T` is `Default + Component`. This bound cannot be defined
/// in the structure definition for now because it would not be possible to define it statically.
pub struct DefaultEntityCodec<T>(PhantomData<*const T>);
unsafe impl<T> Send for DefaultEntityCodec<T> {}
unsafe impl<T> Sync for DefaultEntityCodec<T> {}

impl<T> DefaultEntityCodec<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> EntityCodec for DefaultEntityCodec<T>
where
    T: Default + Component
{

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(<T as Default>::default());
    }

}

pub static VANILLA_ENTITY_CODEC_REF: &'static dyn EntityCodec = &DefaultEntityCodec::<usize>::new();
