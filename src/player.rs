use std::fmt::Debug;
use termion::color;

use crate::{
    item::Items,
    quest::{QuestList, QuestStatus},
    tui::{Drawable, Orientation, Vector, grid::Grid},
};

#[derive(Clone, Debug, Default)]
pub struct Player {
    pub name: &'static str,
    pub backpack: Vec<Items>,
    pub quests: QuestList,
    pub coordinates: Vector,
    pub orientation: Orientation,
}

impl Player {
    pub fn show_name(&self) -> &'static str {
        self.name
    }

    pub fn show_backpack(&self) -> String {
        let mut backpack = String::new();
        for item in &self.backpack {
            backpack.push_str(&item.to_string())
        }

        backpack
    }

    pub fn show_unsolved_quests(&self) -> String {
        let mut unsolved_quests = String::new();
        for quest in &self.quests.list {
            if quest.get_inner_quest().get_status() != &QuestStatus::Finished {
                unsolved_quests.push_str(&quest.to_string())
            }
        }

        unsolved_quests
    }

    pub fn show_all_quests(&self) -> String {
        let mut quests = String::new();
        for quest in &self.quests.list {
            quests.push_str(&quest.to_string())
        }

        quests
    }

    pub fn select_new_active_quest(&mut self) {
        // TODO: method to select needs to be changed later
        if let Some(quest) = self.quests.list.first_mut() {
            quest.get_inner_mut_quest().change_to_active()
        }
    }

    pub fn solve_quest(&mut self) -> Result<(), ()> {
        // find active quest which should be solved next
        let position_active_quest = match self
            .quests
            .list
            .iter()
            .position(|quest| quest.get_inner_quest().get_status() == &QuestStatus::Active)
        {
            None => return Err(()),
            Some(position) => position,
        };

        // check that all necessary items to solve quest are available
        if !self.quests.list[position_active_quest]
            .get_inner_quest()
            .list_necessary_items()
            .iter()
            .all(|necessary_item| self.backpack.contains(necessary_item))
        {
            return Err(());
        }

        // removed used items
        for used_item in self.quests.list[position_active_quest]
            .get_inner_quest()
            .list_necessary_items()
        {
            let index = self
                .backpack
                .iter()
                .position(|available_item| available_item == used_item)
                .expect("Checked before that backpack contains all necessary items");
            self.backpack.remove(index);
        }

        // quest is finished
        self.quests.list[position_active_quest]
            .get_inner_mut_quest()
            .change_to_closed();

        // add rewarded items
        let mut rewarded_items = self.quests.list[position_active_quest]
            .get_inner_quest()
            .list_reward_items();
        self.backpack.append(&mut rewarded_items);

        // add new quest(s)
        let mut new_quests = self.quests.list[position_active_quest]
            .get_inner_quest()
            .list_follow_up_quests();
        self.quests.list.append(&mut new_quests);

        Ok(())
    }

    pub fn move_in_direction(&mut self, grid: &Grid, orientation: Orientation) {
        // avatar rotates to face requested direction
        self.orientation = orientation.clone();

        // check if avatar can make one step
        if let Ok(new_coordinates) = grid.get_new_free_coordinates(&self.coordinates, &orientation)
        {
            self.coordinates = new_coordinates;
        }
    }
}
impl Drawable for Player {
    fn draw(&self) -> String {
        let symbol = match self.orientation {
            Orientation::North => '▲',
            Orientation::East => '▶',
            Orientation::South => '▼',
            Orientation::West => '◀',
        };
        format!(
            "{}{}{}{}",
            termion::cursor::Goto(
                (self.coordinates.horizontal + 1) as u16,
                (self.coordinates.vertical + 1) as u16
            ),
            color::Fg(color::Magenta),
            symbol,
            color::Fg(color::Reset),
        )
    }
}
