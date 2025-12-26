pub mod collected_item;
pub mod reward_items;
pub mod stationed_item;

pub trait Item {
    fn get_name(&self) -> &'static str;
    fn get_description(&self) -> &'static str;
}
