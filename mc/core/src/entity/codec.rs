//! Codec basics for ECS entities' components.

use std::marker::PhantomData;

use hecs::{Component, EntityBuilder, EntityRef};
use nbt::CompoundTag;


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
    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String>;

    /// Decode given source compound tag and add decoded components into the given entity builder.
    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String>;

    /// Add default components to the given entity builder.
    fn default(&self, dst: &mut EntityBuilder);

}


/// This trait is a simpler specification of `EntityCodec` with the restriction of allowing only
/// one component (implementing `hecs::Component`) to be encoded or decoded. The component type
/// also have to implement `Default` to provide a default `EntityCodec::default` implementation.
///
/// Implementing this trait automatically means that you are implementing `EntityCodec` on `Self`,
/// with `encode`, `decode` and `default` automatically defined to delegate to this trait.
pub trait SingleEntityCodec {

    /// Component type, an associated type is used to allow only on implementation per codec struct.
    type Comp: Default + Component;

    fn encode(&self, src: &Self::Comp, dst: &mut CompoundTag);
    fn decode(&self, src: &CompoundTag) -> Self::Comp;

}

impl<C, D> EntityCodec for C
where
    C: Send + Sync,
    C: SingleEntityCodec<Comp=D>,
    D: Default + Component
{

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        if let Some(comp) = src.get::<D>() {
            <Self as SingleEntityCodec>::encode(self, &*comp, dst);
        }
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        dst.add(<Self as SingleEntityCodec>::decode(self, src));
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(D::default());
    }

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

#[allow(unused_variables)]
impl<T> EntityCodec for DefaultEntityCodec<T>
where
    T: Default + Component
{

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) {
        dst.add(<T as Default>::default());
    }

}


/// A null codec, this codec make nothing and can be used as a temporary placeholder.
pub struct NullEntityCodec;

#[allow(unused_variables)]
impl EntityCodec for NullEntityCodec {

    fn encode(&self, src: &EntityRef, dst: &mut CompoundTag) -> Result<(), String> {
        Ok(())
    }

    fn decode(&self, src: &CompoundTag, dst: &mut EntityBuilder) -> Result<(), String> {
        Ok(())
    }

    fn default(&self, dst: &mut EntityBuilder) { }

}
