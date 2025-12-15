use std::fmt::Debug;

use crate::{
    item::Items,
    quest::{MutQuest, Quest, QuestStatus, Quests},
};

#[derive(Clone, Debug, Default)]
pub struct Player {
    pub name: &'static str,
    pub backpack: Vec<Items>,
    pub quests: Vec<Quests>,
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
        for quest in &self.quests {
            if quest.get_inner_quest().get_status() != &QuestStatus::Finished {
                unsolved_quests.push_str(&quest.to_string())
            }
        }

        unsolved_quests
    }

    pub fn show_all_quests(&self) -> String {
        let mut quests = String::new();
        for quest in &self.quests {
            quests.push_str(&quest.to_string())
        }

        quests
    }

    pub fn select_new_active_quest(&mut self) {
        // TODO: method to select needs to be changed later
        if let Some(quest) = self.quests.first_mut() {
            quest.get_inner_mut_quest().change_to_active()
        }
    }

    pub fn solve_quest(&mut self) -> Result<(), ()> {
        // find active quest which should be solved next
        let position_active_quest = match self
            .quests
            .iter()
            .position(|quest| quest.get_inner_quest().get_status() == &QuestStatus::Active)
        {
            None => return Err(()),
            Some(position) => position,
        };

        // check that all necessary items to solve quest are available
        if !self.quests[position_active_quest]
            .get_inner_quest()
            .list_necessary_items()
            .iter()
            .all(|necessary_item| self.backpack.contains(necessary_item))
        {
            return Err(());
        }

        // removed used items
        for used_item in self.quests[position_active_quest]
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

        // quest is finished and returned to backlog
        self.quests[position_active_quest]
            .get_inner_mut_quest()
            .change_to_closed();

        Ok(())
    }
}
