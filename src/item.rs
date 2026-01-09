use crate::inspection::Inspection;

pub mod collected_item;
pub mod stationed_item;

pub trait Item {
    fn get_name(&self) -> &'static str;
    fn get_description(&self) -> &'static str;
    fn get_detail_view(&self) -> String;
    fn get_inspection(&self) -> Inspection {
        Inspection {
            name: String::from(self.get_name()),
            description: String::from(self.get_description()),
            detail_view: self.get_detail_view(),
        }
    }
}
