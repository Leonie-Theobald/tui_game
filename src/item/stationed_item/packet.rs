use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{
            Interaction, InteractionStatus,
            packet_interaction::{Open, PacketInteraction, Shake, Stomp},
        },
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 13,
    vertical: 6,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Packet {
    opened: bool,
    linked_interactions: Vec<Interaction>,
}

impl Packet {
    pub fn initiate() -> Self {
        let linked_interactions = vec![
            Interaction::Packet(PacketInteraction::Open(Open)),
            Interaction::Packet(PacketInteraction::Stomp(Stomp)),
            Interaction::Packet(PacketInteraction::Shake(Shake)),
        ];

        Self {
            opened: false,
            linked_interactions,
        }
    }
}

impl Item for Packet {
    fn get_name(&self) -> &'static str {
        "Wrapped Packet"
    }

    fn get_description(&self) -> &'static str {
        "A small packet wrapped in paper lying on the ground. Take a look inside!"
    }

    fn get_detail_view(&self) -> String {
        if self.opened {
            String::from(
                "                                                               


                       ____________________________                              
                /\\    /                           /
               /  \\  /                           /                              
              /    \\/__________________________ /______                            
             /     /|                          /      /                            
             \\    / |                         /      /                             
              \\  /  |                        /      /                              
               \\/___________________________/______/                               
               /                            \\   |                               
              /                              \\  |
             /________________________________\\ |                               
               |        ███████████        |    |                               
               |        █  PACKET █        |    |                               
               |        ███████████        |    |                               
               |                           |   /                                
               |      HANDLE WITH CARE     |  /                                 
               |                           | /                                  
               |___________________________|/                                 
                                                                                

                                                                                ",
            )
        } else {
            String::from(
                "                                                               
                                                                                

                                                                                
                    ____________________________                                
                   /                           /|       
                  /___________________________/ |                               
                 /                           /  |                               
                /___________________________/   |                               
               |                           |    |                               
               |       SEALED PACKAGE      |    |                               
               |                           |    |                               
               |        ███████████        |    |                               
               |        █  PACKET █        |    |                               
               |        ███████████        |    |                               
               |                           |   /                                
               |      HANDLE WITH CARE     |  /                                 
               |                           | /                                  
               |___________________________|/                                 
                                                                                
                                                                                
                                                                                ",
            )
        }
    }
}

impl Stationary for Packet {
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

impl Drawable for Packet {
    fn draw(&self) -> String {
        let symbol = if self.opened { "X" } else { "📦" };

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
