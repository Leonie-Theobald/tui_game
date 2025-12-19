mod quest;

use termion::color;

use crate::{
    item::{collected_item::CollectedItems, stationed_item::StationedItems},
    quest_log::quest::{IsQuest, Quest, QuestStatus, unwrap_packet::UnwrapPacket},
    tui::Drawable,
};

#[macro_export]
macro_rules! method_on_inner_quest {
    ($value:expr, $method:ident) => {
        match &$value {
            Quest::OpenRustyLock(inner) => inner.$method(),
            Quest::LightCandle(inner) => inner.$method(),
            Quest::UnwrapPacket(inner) => inner.$method(),
        }
    };
}

#[macro_export]
macro_rules! mut_method_on_inner_quest {
    ($value:expr, $method:ident) => {
        match $value {
            Quest::OpenRustyLock(inner) => inner.$method(),
            Quest::LightCandle(inner) => inner.$method(),
            Quest::UnwrapPacket(inner) => inner.$method(),
        }
    };
}

#[derive(Clone, Debug, Default)]
pub struct QuestLog {
    pub list: Vec<Quest>,
    pub visible: bool,
}

impl QuestLog {
    pub fn initiate() -> Self {
        let unwrap_packet = Quest::UnwrapPacket(UnwrapPacket::default());
        Self {
            list: vec![unwrap_packet],
            visible: false,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn get_unsolved_quests(&self) -> Vec<Quest> {
        self.list
            .clone()
            .into_iter()
            .filter(|quest| method_on_inner_quest!(quest, get_status) != &QuestStatus::Finished)
            .collect()
    }

    pub fn select_new_active_quest(&mut self) {
        // TODO: method to select needs to be changed later
        if let Some(quest) = self.list.first_mut() {
            mut_method_on_inner_quest!(quest, change_to_active)
        }
    }

    pub fn solve_quest(
        &mut self,
        collected_items: &mut CollectedItems,
        stationed_items: &StationedItems,
    ) -> Result<(), ()> {
        // find active quest which should be solved next
        let position_active_quest = match self
            .list
            .iter()
            .position(|quest| method_on_inner_quest!(quest, get_status) == &QuestStatus::Active)
        {
            None => return Err(()),
            Some(position) => position,
        };

        // check that all collectable items necessary to solve quest are available
        let necessary_collected_items = method_on_inner_quest!(
            self.list[position_active_quest],
            list_necessary_collected_items
        );
        if !necessary_collected_items
            .iter()
            .all(|necessary_item| collected_items.list.contains(necessary_item))
        {
            return Err(());
        }

        // check that all stationary items necessary to solve quest are available
        let necessary_stationed_items = method_on_inner_quest!(
            self.list[position_active_quest],
            list_necessary_stationed_item
        );
        if !necessary_stationed_items
            .iter()
            .all(|necessary_item| stationed_items.list.contains(necessary_item))
        {
            return Err(());
        }

        // removed used items
        for used_item in method_on_inner_quest!(
            self.list[position_active_quest],
            list_necessary_collected_items
        ) {
            let index = collected_items
                .list
                .iter()
                .position(|available_item| *available_item == used_item)
                .expect("Checked before that backpack contains all necessary items");
            collected_items.list.remove(index);
        }

        // quest is finished
        mut_method_on_inner_quest!(&mut self.list[position_active_quest], change_to_closed);

        // generate rewarded items
        let mut rewarded_items =
            method_on_inner_quest!(self.list[position_active_quest], list_reward_items);
        collected_items.list.append(&mut rewarded_items);

        // generate new quest(s)
        let mut new_quests =
            method_on_inner_quest!(self.list[position_active_quest], list_follow_up_quests);
        self.list.append(&mut new_quests);

        Ok(())
    }
}

impl Drawable for QuestLog {
    fn draw(&self) -> String {
        if !self.visible {
            return String::new();
        }

        let unsolved_quests = self.get_unsolved_quests();

        // format each quest line and find out longest quest line
        let mut length_longest_quest_line = 0;
        let mut joined_quests = String::new();
        for (index, quest) in unsolved_quests.iter().enumerate() {
            let line = format!(
                "{}{}{}{}",
                termion::cursor::Goto(4, 3 + index as u16),
                color::Bg(color::Blue),
                quest,
                color::Bg(color::Reset),
            );
            joined_quests.push_str(&line);

            let description = method_on_inner_quest!(quest, get_description);
            if description.len() > length_longest_quest_line {
                length_longest_quest_line = description.len();
            }
        }

        // format background depending on number of quests and the longest quest line
        // line length depends on quests length
        let background_line = format!(
            "{}{}{}",
            color::Bg(color::Blue),
            " ".repeat(length_longest_quest_line + 4),
            color::Bg(color::Reset),
        );

        // height depends on number of quests
        let mut background = String::new();
        for index in 0..unsolved_quests.len() + 2 {
            background.push_str(&format!(
                "{}{}",
                termion::cursor::Goto(2, 2 + index as u16),
                background_line
            ))
        }

        // merge background and written quests
        format!("{}{}", background, joined_quests)
    }
}
