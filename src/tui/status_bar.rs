use crate::{
    item::collected_item::{Collectable, CollectableItem, CollectedItems},
    method_on_inner_collected_item,
    tui::grid::GRID_HEIGHT,
};

pub fn draw_status_bar(collected_items: &CollectedItems) -> String {
    let formatted_line_with_collected_items =
        collected_items
            .list
            .iter()
            .fold(String::new(), |mut joined_line, item| {
                let symbol = method_on_inner_collected_item!(item, get_symbol);
                joined_line.push_str(&format!("{}  ", &symbol,));
                joined_line
            });

    format!(
        "{}{} {} {}",
        termion::cursor::Goto(1, (GRID_HEIGHT + 1) as u16),
        termion::color::Bg(termion::color::LightBlue),
        termion::color::Bg(termion::color::Reset),
        formatted_line_with_collected_items,
    )
}
