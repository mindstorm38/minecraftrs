use super::Item;
use nbt::CompoundTag;


pub struct ItemStack {
    item: &'static Item,
    count: u16,
    tag: Option<CompoundTag>
}

impl ItemStack {

    #[inline]
    pub fn new(item: &'static Item, count: u16, tag: Option<CompoundTag>) -> Self {
        debug_assert_ne!(count, 0, "Item stack can't have a count of zero.");
        Self {
            item,
            count,
            tag
        }
    }

    pub fn with_item(item: &'static Item) -> Self {
        Self::new(item, 1, None)
    }

    pub fn with_item_count(item: &'static Item, count: u16) -> Self {
        Self::new(item, count, None)
    }

    #[inline]
    pub fn get_item(&self) -> &'static Item {
        self.item
    }

    #[inline]
    pub fn get_count(&self) -> u16 {
        self.count
    }

    #[inline]
    pub fn get_tag(&self) -> Option<&CompoundTag> {
        self.tag.as_ref()
    }

    pub fn get_tag_mut(&mut self) -> &mut CompoundTag {
        self.tag.get_or_insert_with(|| CompoundTag::new())
    }

}
