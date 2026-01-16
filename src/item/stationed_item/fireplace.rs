use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{
            Interaction, InteractionStatus,
            fireplace_interaction::{
                FireplaceInteraction, LightBranch, RoastMarshmallow, ThrowBranchIntoFire,
            },
        },
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 38,
    vertical: 10,
};
const DIMENSIONS: Vector = Vector {
    horizontal: 6,
    vertical: 4,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fireplace {
    linked_interactions: Vec<Interaction>,
}

impl Fireplace {
    pub fn initiate() -> Self {
        Self {
            linked_interactions: vec![
                Interaction::Fireplace(FireplaceInteraction::ThrowBranchIntoFire(
                    ThrowBranchIntoFire,
                )),
                Interaction::Fireplace(FireplaceInteraction::LightBranch(LightBranch)),
                Interaction::Fireplace(FireplaceInteraction::RoastMarshmallow(RoastMarshmallow)),
            ],
        }
    }
}

impl Item for Fireplace {
    fn get_name(&self) -> &'static str {
        "Cosy Fireplace"
    }

    fn get_description(&self) -> &'static str {
        "The fireplace is burning."
    }

    fn get_detail_view(&self) -> String {
        String::from(
            "

 .___________________________________________________________________________. 
 |                                                                           | 
 |___________________________________________________________________________| 
   |     !           |           |           |           |           |     |   
   |_____|__,________|___________|______.____}__,________}___________|_____|   
   |         |  \\   \\   \\   \\   \\    |   |    /   /   /   /   /  |         |   
   |_________|___\\_,_\\___\\___\\.__|___|___|___|___/___/___/___/___|_________|   
   |     |   |      |     \\/ ,\".). ',     \\/. ',          |      |   |     |   
   |_____|___|      |. ' ,'|// / (/ ,.  ' ,\\(   / ,       |      |___|_____|   
   |         |     ,|  '|\\/ \\|  \\\\,' ,'.' ||  \\\\,' .'    ,|  '   |         |   
   |_._______|   \" ,|' .| \\_ |\\/ |#\\_/#| /_|\\_ |#\\_/,) ',.|,'    |_________|   
   |     |   |   #|(,'  )  \\\\\\#\\ \\##/  ||/ \\#\\  \\###/\\ \\/ \\/.',  |   |     |   
   |_____|__,|   || ) | \\ |/ /#/ |#( \\|\\\\|  |#/ /##(  )|  \\\\'/   |___|_____|   
   |         |    \\ |.\\\\ |\\_/#| /#)|##\\ \\\\_/#| |######\\ \\\\/#|    |         |   
   |_________|    /\\\\_/# |#\\##/# \\#/     \\|/ /  @\\# #@#\\ /#/\\    |_____.___|   
   |     |   |   /  ##/#  ##\\/#@####@@###V###\\/#@####@@#Y### \\   |   |     |   
   |___._|___|  / :Y##@@##@##@@###@@@###@##@@###@@@###@####:  \\  |___|_____|   
   |         | / :;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:  \\ |         |   
   |_________|/    ::::::::::::::::::::::::::::::::             \\|_________|   ",
        )
    }
}

impl Stationary for Fireplace {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        let mut blocked_coordinates = Vec::new();
        for horizontal_offset in 0..DIMENSIONS.horizontal {
            for vertical_offset in 0..DIMENSIONS.vertical {
                blocked_coordinates.push(Vector {
                    horizontal: COORDINATES.horizontal + horizontal_offset,
                    vertical: COORDINATES.vertical + vertical_offset,
                });
            }
        }

        blocked_coordinates
    }

    fn get_linked_interactions(&self) -> &Vec<Interaction> {
        &self.linked_interactions
    }

    fn get_interaction_status(&self) -> InteractionStatus {
        InteractionStatus::Open
    }

    fn set_interaction_status_to_finished(&mut self) {}
}

impl Drawable for Fireplace {
    fn draw(&self) -> String {
        let fireplace = "🧱🧱🧱
🪵🔥🧱
🪵🔥🧱
🧱🧱🧱";

        let mut fireplace_string = String::new();
        for (index, line) in fireplace.lines().enumerate() {
            fireplace_string.push_str(&format!(
                "{}{}",
                termion::cursor::Goto(
                    (COORDINATES.horizontal + 1) as u16,
                    (COORDINATES.vertical + index + 1) as u16
                ),
                line
            ))
        }

        fireplace_string
    }
}
