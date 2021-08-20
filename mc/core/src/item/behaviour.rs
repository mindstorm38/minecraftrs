use crate::block::Block;
use super::ItemStack;


pub trait ItemBehaviour {
    fn interact(&self, context: ItemInteractContext);
}


pub struct ItemInteractContext<'item> {
    // level: &'level LevelAccess
    item_stack: &'item mut ItemStack
}


pub struct BlockItemBehaviour {
    block: &'static Block,
}

impl ItemBehaviour for BlockItemBehaviour {

    fn interact(&self, context: ItemInteractContext) {

    }

}
