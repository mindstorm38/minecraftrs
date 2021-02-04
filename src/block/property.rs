use std::any::Any;


/// Trait for all properties stored in a block state.
pub trait Property<T: Copy>: Any {
    fn get_name(&self) -> &'static str;
    fn iter_values(&self) -> Box<dyn Iterator<Item=T>>;
    fn encode_prop(&self, value: T) -> u8;
    fn decode_prop(&self, raw: u8) -> Option<T>;
}


pub struct BoolProperty(pub &'static str);

impl Property<bool> for BoolProperty {

    fn get_name(&self) -> &'static str {
        self.0
    }

    fn iter_values(&self) -> Box<dyn Iterator<Item = bool>> {
        static VALUES: [bool; 2] = [false, true];
        Box::new(VALUES.iter().copied())
    }

    fn encode_prop(&self, value: bool) -> u8 {
        if value { 1 } else { 0 }
    }

    fn decode_prop(&self, raw: u8) -> Option<bool> {
        Some(raw != 0)
    }

}


pub struct IntProperty(pub &'static str, pub u8);

impl Property<u8> for IntProperty {

    fn get_name(&self) -> &'static str {
        self.0
    }

    fn iter_values(&self) -> Box<dyn Iterator<Item = u8>> {
        Box::new((0..self.1).into_iter())
    }

    fn encode_prop(&self, value: u8) -> u8 {
        value
    }

    fn decode_prop(&self, raw: u8) -> Option<u8> {
        if raw < self.1 {
            Some(raw)
        } else {
            None
        }
    }

}


pub struct EnumProperty<T: 'static + Copy + Eq>(pub &'static str, pub &'static [T]);

impl<T> Property<T> for EnumProperty<T>
where
    T: 'static + Copy + Eq
{

    fn get_name(&self) -> &'static str {
        self.0
    }

    fn iter_values(&self) -> Box<dyn Iterator<Item=T>> {
        Box::new(self.1.iter().copied())
    }

    fn encode_prop(&self, value: T) -> u8 {
        self.1.iter().position(|v| *v == value).unwrap() as u8
    }

    fn decode_prop(&self, raw: u8) -> Option<T> {
        self.1.get(raw as usize).copied()
    }

}
