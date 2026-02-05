pub fn room_with_cake() -> String {
    let uncolored_background =
        "wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg";

    let mut colored_background = String::new();
    for (vertical_offset, line) in uncolored_background.lines().enumerate() {
        for (horizontal_offset, char) in line.chars().enumerate() {
            match char {
                'w' => colored_background.push_str(&format!(
                    "{}{} {}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(250)),
                    termion::color::Bg(termion::color::Reset),
                )),
                'g' => colored_background.push_str(&format!(
                    "{}{} {}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(59)),
                    termion::color::Bg(termion::color::Reset),
                )),
                _ => unreachable!(),
            }
        }
    }

    let uncolored_table =
        "................................................................................
................................................................................
................................................................................
................................................................................
................................................................................
................................................................................
................................................................................
................................................................................
...............................-------------------------........................
............................../bbbbbbbbbbbbbbbbbbbbbbbb/........................
............................./bbbbbbbbbbbbbbbbbbbbbbbb/.........................
............................/bbbbbbbbbbbbbbbbbbbbbbbb/..........................
.........................../bbbbbbbbbbbbbbbbbbbbbbbb/|..........................
........................../________________________/||..........................
.............................||................||...............................
.............................||................||...............................
.............................||................||...............................
.............................||................||...............................
.............................||................||...............................
................................................................................
................................................................................
................................................................................";

    let mut colored_table = String::new();
    for (vertical_offset, line) in uncolored_table.lines().enumerate() {
        for (horizontal_offset, char) in line.chars().enumerate() {
            match char {
                '.' => continue,
                'b' => colored_table.push_str(&format!(
                    "{}{} {}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(130)),
                    termion::color::Bg(termion::color::Reset),
                )),
                c @ ('/' | '_' | '-' | '|') => colored_table.push_str(&format!(
                    "{}{}{}{}{}{}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(130)),
                    termion::color::Fg(termion::color::Black),
                    c,
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset),
                )),
                _ => unreachable!(),
            }
        }
    }

    let uncolored_cake =
        "................................................................................
................................................................................
................................................................................
................................................................................
................................................................................
................................................................................
................................................................................
....................................--*--.......................................
.................................../ww█ww\\......................................
...................................|\\____/......................................
...................................|wwwww|......................................
...................................\\_____/......................................
................................................................................
................................................................................
................................................................................
................................................................................
................................................................................
................................................................................
................................................................................
................................................................................
................................................................................
................................................................................";

    let mut colored_cake = String::new();
    for (vertical_offset, line) in uncolored_cake.lines().enumerate() {
        for (horizontal_offset, char) in line.chars().enumerate() {
            match char {
                '.' => continue,
                'w' => colored_cake.push_str(&format!(
                    "{}{} {}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(224)),
                    termion::color::Bg(termion::color::Reset),
                )),
                c @ ('/' | '\\' | '_' | '-' | '|') => colored_cake.push_str(&format!(
                    "{}{}{}{}{}{}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(224)),
                    termion::color::Fg(termion::color::AnsiValue(137)),
                    c,
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset),
                )),
                '█' => colored_cake.push_str(&format!(
                    "{}{}{}█{}{}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(224)),
                    termion::color::Fg(termion::color::AnsiValue(160)),
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset),
                )),
                '*' => colored_cake.push_str(&format!(
                    "{}{}{}({}{}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(224)),
                    termion::color::Fg(termion::color::AnsiValue(214)),
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset),
                )),
                _ => unreachable!(),
            }
        }
    }

    let the_cake_is_a_lie = format!(
        "{}{}{}The cake is a lie{}{}",
        termion::cursor::Goto(52, 22),
        termion::color::Bg(termion::color::AnsiValue(59)),
        termion::color::Fg(termion::color::AnsiValue(243)),
        termion::color::Bg(termion::color::Reset),
        termion::color::Fg(termion::color::Reset)
    );

    // piece together scenery
    format!(
        "{}{}{}{}",
        colored_background, colored_table, colored_cake, the_cake_is_a_lie
    )
}
