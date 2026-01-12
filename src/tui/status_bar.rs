use crate::{
    item::{
        collected_item::{Collectable, CollectableItem, CollectedItems},
        stationed_item::StationedItems,
    },
    method_on_inner_collected_item,
    player::Player,
    quest::Quests,
    tui::grid::{GRID_HEIGHT, GRID_WIDTH},
};

pub fn draw_status_bar(
    stationed_items: &StationedItems,
    collected_items: &CollectedItems,
    player: &Player,
    quests: &Quests,
) -> String {
    // show how many interactions are possible
    let target_coordinates = player.get_facing_coordinates();
    let interaction_count = stationed_items.count_possible_interactions(&target_coordinates);
    let interaction_text = if interaction_count == 1 {
        String::from("int<e>raction ")
    } else {
        String::from("int<e>ractions")
    };
    let joined_interaction_line = format!("{:2} {}", interaction_count, interaction_text);

    // show how many collected items are available
    let collected_items_count = collected_items.list.len();
    let collection_text = String::from("<c>ollected ");
    let joined_collection_line = format!("{:2} {}", collected_items_count, collection_text);

    // show how many quests are open
    let quest_count = quests
        .list
        .iter()
        .filter(|quest| !quest.get_inner().is_finished())
        .count();
    let quest_text = if quest_count == 1 {
        String::from("<q>uest ")
    } else {
        String::from("<q>uests")
    };
    let joined_quest_line = format!("{:2} {}", quest_count, quest_text);

    // show exit game key binding
    let exit_text = "Exit: <ESC>";
    let exit_text_line = format!(
        "{}{}",
        termion::cursor::Goto(
            (GRID_WIDTH - exit_text.len()) as u16,
            (GRID_HEIGHT + 2) as u16
        ),
        exit_text
    );

    // display symbols of currently collected items
    let joined_collected_items =
        collected_items
            .list
            .iter()
            .fold(String::new(), |mut joined_line, item| {
                let symbol = method_on_inner_collected_item!(item, get_symbol);
                joined_line.push_str(&format!("{}  ", &symbol,));
                joined_line
            });

    // bring everything together
    format!(
        "{} {} {} {} {} {}{}",
        termion::cursor::Goto(1, (GRID_HEIGHT + 1) as u16),
        joined_collected_items,
        termion::cursor::Goto(1, (GRID_HEIGHT + 2) as u16),
        joined_interaction_line,
        joined_quest_line,
        joined_collection_line,
        exit_text_line,
    )
}
