use termion::color;

use crate::{
    item::collected_item::{
        CollectableItem, CollectedItems, candle::Candle, rusty_key::RustyKey,
        treasure_map::TreasureMap,
    },
    tui::Drawable,
};

macro_rules! method_on_inner_quest {
    ($value:expr, $method:ident) => {
        match &$value {
            Quests::OpenRustyLock(inner) => inner.$method(),
            Quests::LightCandle(inner) => inner.$method(),
        }
    };
}

macro_rules! mut_method_on_inner_quest {
    ($value:expr, $method:ident) => {
        match $value {
            Quests::OpenRustyLock(inner) => inner.$method(),
            Quests::LightCandle(inner) => inner.$method(),
        }
    };
}

#[derive(Clone, Debug, Default)]
pub struct QuestList {
    pub list: Vec<Quests>,
    pub visible: bool,
}

impl QuestList {
    pub fn initiate() -> Self {
        let open_rusty_lock = Quests::OpenRustyLock(OpenRustyLock::default());
        Self {
            list: vec![open_rusty_lock],
            visible: false,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn get_unsolved_quests(&self) -> Vec<Quests> {
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

    pub fn solve_quest(&mut self, collected_items: &mut CollectedItems) -> Result<(), ()> {
        // find active quest which should be solved next
        let position_active_quest = match self
            .list
            .iter()
            .position(|quest| method_on_inner_quest!(quest, get_status) == &QuestStatus::Active)
        {
            None => return Err(()),
            Some(position) => position,
        };

        // check that all necessary items to solve quest are available
        let necessary_items =
            method_on_inner_quest!(self.list[position_active_quest], list_necessary_items);
        if !necessary_items
            .iter()
            .all(|necessary_item| collected_items.list.contains(necessary_item))
        {
            return Err(());
        }

        // removed used items
        for used_item in
            method_on_inner_quest!(self.list[position_active_quest], list_necessary_items)
        {
            let index = collected_items
                .list
                .iter()
                .position(|available_item| available_item == used_item)
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

impl Drawable for QuestList {
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

pub trait Quest {
    fn get_description(&self) -> &'static str;
    fn list_necessary_items(&self) -> &[CollectableItem];
    fn get_status(&self) -> &QuestStatus;
    fn list_reward_items(&self) -> Vec<CollectableItem>;
    fn list_follow_up_quests(&self) -> Vec<Quests>;
    fn change_to_closed(&mut self);
    fn change_to_active(&mut self);
}

#[derive(Clone, Debug)]
pub enum Quests {
    OpenRustyLock(OpenRustyLock),
    LightCandle(LightCandle),
}

impl std::fmt::Display for Quests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", method_on_inner_quest!(self, get_description))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum QuestStatus {
    #[default]
    Open,
    Active,
    Finished,
}

#[derive(Clone, Debug, Default)]
pub struct OpenRustyLock {
    status: QuestStatus,
}

impl Quest for OpenRustyLock {
    fn get_description(&self) -> &'static str {
        "This old, wooden door is locked. Open it"
    }

    fn list_necessary_items(&self) -> &[CollectableItem] {
        &[CollectableItem::RustyKey(RustyKey)]
    }

    fn get_status(&self) -> &QuestStatus {
        &self.status
    }

    fn list_reward_items(&self) -> Vec<CollectableItem> {
        vec![
            CollectableItem::Candle(Candle),
            CollectableItem::TreasureMap(TreasureMap),
        ]
    }

    fn list_follow_up_quests(&self) -> Vec<Quests> {
        vec![Quests::LightCandle(LightCandle::default())]
    }

    fn change_to_closed(&mut self) {
        self.status = QuestStatus::Finished;
    }

    fn change_to_active(&mut self) {
        self.status = QuestStatus::Active;
    }
}

#[derive(Clone, Debug, Default)]
pub struct LightCandle {
    status: QuestStatus,
}

impl Quest for LightCandle {
    fn get_description(&self) -> &'static str {
        "The candle is not burning. Try to lighten it."
    }

    fn list_necessary_items(&self) -> &[CollectableItem] {
        &[CollectableItem::Candle(Candle)]
    }

    fn get_status(&self) -> &QuestStatus {
        &self.status
    }

    fn list_reward_items(&self) -> Vec<CollectableItem> {
        Vec::new()
    }

    fn list_follow_up_quests(&self) -> Vec<Quests> {
        Vec::new()
    }

    fn change_to_closed(&mut self) {
        self.status = QuestStatus::Finished;
    }

    fn change_to_active(&mut self) {
        self.status = QuestStatus::Active;
    }
}
