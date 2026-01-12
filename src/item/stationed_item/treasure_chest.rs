use crate::{
    item::{
        Item,
        collected_item::{
            CollectableItem, CollectedItems, candle::Candle, chest_key::ChestKey,
            treasure_map::TreasureMap,
        },
        interaction::{Interactable, Interaction, InteractionStatus},
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector},
};
use serde::{Deserialize, Serialize};

const COORDINATES: Vector = Vector {
    horizontal: 45,
    vertical: 16,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TreasureChest {
    opened: bool,
    linked_interactions: Vec<Interaction>,
}

impl TreasureChest {
    pub fn initiate() -> Self {
        let linked_interactions = vec![Interaction::TreasureChest(
            TreasureChestInteraction::Unlock(Unlock {}),
        )];

        Self {
            opened: false,
            linked_interactions,
        }
    }
}

impl Item for TreasureChest {
    fn get_name(&self) -> &'static str {
        "Treasure Chest"
    }

    fn get_description(&self) -> &'static str {
        "Open this treasure chest with a key."
    }

    fn get_detail_view(&self) -> String {
        if self.opened {
            String::from(
                "
                                                                                
                                                                                
                                                                                
                 x                                                              
                                                                                
                                                                                
                      _______________________________                                                          
                      \\                              \\                           
                       \\______________________________\\
                       /|                            /|
                      /-┵---------------------------/ |
                      |  ┏━━━━━━━━━━━━━━━━━━━━━━━┓  | |
                      |  ┃        ╔═════╗        ┃  | |
                      |  ┃        ║  o  ║        ┃  | |
                      |  ┃        ╚═════╝        ┃  | |
                      |  ┃                       ┃  | |
                      |  ┃                       ┃  | |
                      |  ┗━━━━━━━━━━━━━━━━━━━━━━━┛  | |
                      |_____________________________|/
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                ",
            )
        } else {
            String::from(
                "
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                        ______________________________
                       /                             /|
                      /-----------------------------/ |
                      |  ┏━━━━━━━━━━━━━━━━━━━━━━━┓  | |
                      |  ┃        ╔═════╗        ┃  | |
                      |  ┃        ║  o  ║        ┃  | |
                      |  ┃        ╚═════╝        ┃  | |
                      |  ┃                       ┃  | |
                      |  ┃                       ┃  | |
                      |  ┗━━━━━━━━━━━━━━━━━━━━━━━┛  | |
                      |_____________________________|/
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                ",
            )
        }
    }
}

impl Stationary for TreasureChest {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        let mut blocked_coordinate = Vec::new();

        // +-1 around the symbol is also blocked
        let horizontal_range = COORDINATES.horizontal..COORDINATES.horizontal + 2;
        for horizontal_index in horizontal_range {
            let vertical_range = COORDINATES.vertical..COORDINATES.vertical + 1;
            for vertical_index in vertical_range {
                blocked_coordinate.push(Vector {
                    horizontal: horizontal_index,
                    vertical: vertical_index,
                })
            }
        }

        blocked_coordinate
    }

    fn get_linked_interactions(&self) -> &Vec<Interaction> {
        &self.linked_interactions
    }

    fn get_interaction_status(&self) -> InteractionStatus {
        if self.opened {
            InteractionStatus::Finished
        } else {
            InteractionStatus::Open
        }
    }

    fn set_interaction_status_to_finished(&mut self) {
        self.opened = true;
    }
}

impl Drawable for TreasureChest {
    fn draw(&self) -> String {
        let symbol = if self.opened { "X" } else { "🧳" };

        format!(
            "{}{}",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16
            ),
            symbol,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TreasureChestInteraction {
    Unlock(Unlock),
}

impl TreasureChestInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            TreasureChestInteraction::Unlock(unlock) => unlock,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unlock;
impl Interactable for Unlock {
    fn get_prompt(&self) -> String {
        String::from("Use key to open chest")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        if !collected_items
            .list
            .contains(&CollectableItem::ChestKey(ChestKey))
        {
            return String::from("You have no key to open the treasure chest.");
        }

        // remove key
        let index = collected_items
            .list
            .iter()
            .position(|available_item| *available_item == CollectableItem::ChestKey(ChestKey))
            .expect("Checked before that item is available");
        collected_items.list.remove(index);

        // add collectable items
        collected_items.list.push(CollectableItem::Candle(Candle));
        collected_items
            .list
            .push(CollectableItem::TreasureMap(TreasureMap));

        // finish interaction status
        linked_stationary_item.set_interaction_status_to_finished();

        String::from("This was the correct key to use.")
    }
}
