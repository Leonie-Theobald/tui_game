use crate::item::{Item, collected_item::Collectable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Branch;

impl Item for Branch {
    fn get_name(&self) -> &'static str {
        "Branch"
    }

    fn get_description(&self) -> &'static str {
        "A dry and straight branch"
    }

    fn get_detail_view(&self) -> String {
        String::from(
            "


                         _          
                        //
                       //
                      //
                _____/ __
                    / / \\   
                   / /___/   
             \\___/ /_ \\         
                /  /   _ 
              _/  /___/___/    
            _/  _/   \\ 
        ___/ __/ 
        \\___/",
        )
    }
}

impl Collectable for Branch {
    fn get_symbol(&self) -> String {
        String::from("🪾")
    }
}
