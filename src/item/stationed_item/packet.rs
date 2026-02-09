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
    tui::{Drawable, Vector, background::BACKGROUND_COLOR},
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
        let uncolored_packet = if self.opened {
            "                                                               


                       ____________________________                              
                /\\    /.........................../
               /..\\  /.........................../                              
              /....\\/__________________________./______                            
             /...../|....................B.=.5./....../                            
             \\..../.|........................./....../                             
              \\../..|......................../....../                              
               \\/___________________________/______/                               
               /............................\\...|                               
              /..............................\\..|
             /________________________________\\.|                               
               |........wwwwwwwwww.........|....|                               
               |........wwPACKETww.........|....|                               
               |........wwwwwwwwww.........|....|                               
               |...........................|.../                                
               |.....wHANDLEwWITHwCAREw....|../                                 
               |...........................|./                                  
               |___________________________|/                                 
                                                                                

                                                                                "
        } else {
            "                                                               
                                                                                

                                                                                
                    ____________________________                                
                   /.........................../|       
                  /___________________________/.|                               
                 /.........................../..|                               
                /___________________________/...|                               
               |...........................|....|                               
               |......wSEALEDwPACKAGEw.....|....|                               
               |...........................|....|                               
               |........wwwwwwwwww.........|....|                               
               |........wwPACKETww.........|....|                               
               |........wwwwwwwwww.........|....|                               
               |...........................|.../                                
               |.....wHANDLEwWITHwCAREw....|../                                 
               |...........................|./                                  
               |___________________________|/                                 
                                                                                
                                                                                
                                                                                "
        };

        let mut colored_packet = String::new();
        for (vertical_offset, line) in uncolored_packet.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    // encode one password letter for `Terminal`
                    c @ ('B' | '=' | '5') => colored_packet.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::AnsiValue(130)),
                        termion::color::Bg(termion::color::AnsiValue(137)),
                        c,
                        termion::color::Fg(termion::color::Reset),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    'w' => colored_packet.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::White),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    '.' => colored_packet.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(137)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c if c.is_alphabetic() => colored_packet.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::White),
                        termion::color::Fg(termion::color::Black),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                    )),
                    c => colored_packet.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(137)),
                        termion::color::Fg(termion::color::Black),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                    )),
                }
            }
        }

        colored_packet
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
        format!(
            "{}{}📦{}",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16
            ),
            termion::color::Bg(BACKGROUND_COLOR),
            termion::color::Bg(termion::color::Reset),
        )
    }
}
