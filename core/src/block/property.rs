use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::str::FromStr;
use std::any::Any;


/// All valid property values types must implement this trait.
pub trait PropertySerializable: 'static + Copy {
    fn prop_to_string(self) -> String;
    fn prop_from_string(value: &str) -> Option<Self>;
}


/// An untyped property trait used for storage in shared property.
pub trait UntypedProperty: Any + Sync {
    fn name(&self) -> &'static str;
    fn len(&self) -> u8;
    fn prop_to_string(&self, index: u8) -> Option<String>;
    fn prop_from_string(&self, value: &str) -> Option<u8>;
}


/// Trait for all properties stored in a block state.
pub trait Property<T: PropertySerializable>: UntypedProperty {
    fn encode(&self, value: T) -> Option<u8>;
    fn decode(&self, index: u8) -> Option<T>;
}


impl<T> PropertySerializable for T
    where
        T: 'static + Copy + ToString + FromStr
{

    fn prop_to_string(self) -> String {
        self.to_string()
    }

    fn prop_from_string(value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }

}


pub struct BoolProperty(pub &'static str);

impl UntypedProperty for BoolProperty {

    fn name(&self) -> &'static str { self.0 }
    fn len(&self) -> u8 { 2 }

    fn prop_to_string(&self, index: u8) -> Option<String> {
        Some((index != 0).to_string())
    }

    fn prop_from_string(&self, value: &str) -> Option<u8> {
        Some(if bool::from_str(value).ok()? { 1 } else { 0 })
    }

}

impl Property<bool> for BoolProperty {
    fn encode(&self, value: bool) -> Option<u8> {
        Some(if value { 1 } else { 0 })
    }
    fn decode(&self, index: u8) -> Option<bool> {
        Some(index != 0)
    }
}


pub struct IntProperty(pub &'static str, pub u8);

impl UntypedProperty for IntProperty {

    fn name(&self) -> &'static str { self.0 }
    fn len(&self) -> u8 { self.1 }

    fn prop_to_string(&self, index: u8) -> Option<String> {
        if index < self.1 { Some(index.to_string()) } else { None }
    }

    fn prop_from_string(&self, value: &str) -> Option<u8> {
        let value = u8::from_str(value).ok()?;
        if value < self.1 { Some(value) } else { None }
    }

}

impl Property<u8> for IntProperty {

    fn encode(&self, value: u8) -> Option<u8> {
        if value < self.1 { Some(value) } else { None }
    }

    fn decode(&self, index: u8) -> Option<u8> {
        if index < self.1 { Some(index) } else { None }
    }
}


pub struct EnumProperty<T: PropertySerializable + Eq>(pub &'static str, pub &'static [T]);

impl<T> UntypedProperty for EnumProperty<T>
    where
        T: PropertySerializable + Eq + Sync
{

    fn name(&self) -> &'static str { self.0 }
    fn len(&self) -> u8 { self.1.len() as u8 }

    fn prop_to_string(&self, index: u8) -> Option<String> {
        Some(self.1.get(index as usize)?.prop_to_string())
    }

    fn prop_from_string(&self, value: &str) -> Option<u8> {
        let value = T::prop_from_string(value)?;
        Some(self.1.iter().position(|v| *v == value)? as u8)
    }

}

impl<T> Property<T> for EnumProperty<T>
    where
        T: PropertySerializable + Eq + Sync
{

    fn encode(&self, value: T) -> Option<u8> {
        Some(self.1.iter().position(|v| *v == value)? as u8)
    }

    fn decode(&self, index: u8) -> Option<T> {
        Some(*(self.1.get(index as usize)?))
    }

}


impl Debug for dyn UntypedProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("UntypedProperty").field("name", &self.name()).finish()
    }
}


#[macro_export]
macro_rules! properties {
    ($v:vis $id:ident: int($name:literal, $count:expr); $($t:tt)*) => {
        $v static $id: $crate::block::IntProperty = $crate::block::IntProperty($name, $count);
        $crate::properties!($($t)*);
    };
    ($v:vis $id:ident: bool($name:literal); $($t:tt)*) => {
        $v static $id: $crate::block::BoolProperty = $crate::block::BoolProperty($name);
        $crate::properties!($($t)*);
    };
    ($v:vis $id:ident: enum<$enum_type:ty>($name:literal, $values_id:ident, [$($value:ident),+]); $($t:tt)*) => {
        static $values_id: [$enum_type; 0 $(+ (1, <$enum_type>::$value).0)+] = [$(<$enum_type>::$value),+];
        $v static $id: $crate::block::EnumProperty<$enum_type> = $crate::block::EnumProperty($name, &$values_id);
        $crate::properties!($($t)*);
    };
    ($v:vis $id:ident: enum<$enum_type:ty>($name:literal, $values_id:ident); $($t:tt)*) => {
        $v static $id: $crate::block::EnumProperty<$enum_type> = $crate::block::EnumProperty($name, &$values_id);
        $crate::properties!($($t)*);
    };
    () => {}
}


#[macro_export]
macro_rules! impl_enum_serializable {
    ($enum_id:ident { $($item_id:ident: $item_name:literal),* }) => {
        impl $crate::block::PropertySerializable for $enum_id {
            fn prop_to_string(self) -> String {
                match self {
                    $(Self::$item_id => $item_name),*
                }.to_string()
            }
            fn prop_from_string(value: &str) -> Option<Self> {
                match value {
                    $($item_name => Some(Self::$item_id),)*
                    _ => None
                }
            }
        }
    };
}