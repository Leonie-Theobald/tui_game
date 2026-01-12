use crate::item::{Item, collected_item::Collectable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChestKey;

impl Item for ChestKey {
    fn get_name(&self) -> &'static str {
        "Chest Key"
    }

    fn get_description(&self) -> &'static str {
        "This key belongs to a treasure chest."
    }

    fn get_detail_view(&self) -> String {
        String::from(
            "
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                          ████                                  
                                         █    █                                
                                        █      █                                
                                        █      █                                
              ███████████████████████████      █                                
              █ █                       █      █                                
                ▘                       █      █                                
                                         █    █                                 
                                          ████                                  
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                ",
        )
    }
}

impl Collectable for ChestKey {
    fn get_symbol(&self) -> String {
        String::from("🗝️")
    }
}
