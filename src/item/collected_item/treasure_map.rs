use crate::{
    item::{Item, collected_item::Collectable},
    tui::background::BACKGROUND_COLOR,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TreasureMap;

impl Item for TreasureMap {
    fn get_name(&self) -> &'static str {
        "Treasure Map"
    }

    fn get_description(&self) -> &'static str {
        "A map with a red X on it."
    }

    fn get_detail_view(&self) -> String {
        let uncolored_map = "
           
      %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
      %%..........................................................%%
      %%..............follow.this..\\.....StEp.WhErE.thE...........%%
      %%..............hidden.path../..NExt.aNSWEr.Will.bE.........%%
      %%..........................................................%%
      %%..........................................................%%
      %%..........................................................%%
      %%................────┰─────────────────────────┐...........%%
      %%....................│........██..............╲╱...........%%
      %%....................│.........▲..............╱╲...........%%
      %%....................│.........................│...........%%
      %%....................│.........................│...........%%
      %%....................│.........................│...........%%
      %%.....N..............│..........O..............│...........%%
      %%.....⇑..........────┴────....──┸──────────────┨...........%%
      %%.W.⇐...⇒.E....................................│...........%%
      %%.....⇓....................................................%%
      %%.....S....................................................%%
      %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
";

        let mut colored_map = String::new();

        for (vertical_offset, line) in uncolored_map.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    '.' => colored_map.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(179)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    '%' => colored_map.push_str(&format!(
                        "{}{}{}%{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::AnsiValue(130)),
                        termion::color::Bg(termion::color::AnsiValue(179)),
                        termion::color::Fg(termion::color::Reset),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    '█' => colored_map.push_str(&format!(
                        "{}{}{}█{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::AnsiValue(94)),
                        termion::color::Bg(termion::color::AnsiValue(179)),
                        termion::color::Fg(termion::color::Reset),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    '▲' => colored_map.push_str(&format!(
                        "{}{}{}▲{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::Magenta),
                        termion::color::Bg(termion::color::AnsiValue(179)),
                        termion::color::Fg(termion::color::Reset),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c if ['╲', '╱'].contains(&c) => colored_map.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::AnsiValue(124)),
                        termion::color::Bg(termion::color::AnsiValue(179)),
                        c,
                        termion::color::Fg(termion::color::Reset),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c => colored_map.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::Black),
                        termion::color::Bg(termion::color::AnsiValue(179)),
                        c,
                        termion::color::Fg(termion::color::Reset),
                        termion::color::Bg(termion::color::Reset),
                    )),
                }
            }
        }
        colored_map
    }
}

impl Collectable for TreasureMap {
    fn get_symbol(&self) -> String {
        format!(
            "{}{}{}",
            termion::color::Bg(BACKGROUND_COLOR),
            "📜",
            termion::color::Bg(termion::color::Reset)
        )
    }
}
