use std::io::{Write, stdout};
use termion::raw::IntoRawMode;

pub fn draw_cake() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    // "                          (
    //                   )      (*)                    (
    //                  (*)      |                    (*)
    //                   |      |~|                    |
    //                 ,|~|a@@@@| |@@@@@@@@@@@@@@@@@@a|~|.
    //            .,a@@@| |@@@@@| |@@@@@@@@@@@@@@@@@@@| |@@@@a,.
    // bbbbbbbbb,a@@@@@@| |@@@@@@@@@@@@.@@@@@@@@@@@@@@| |@@@@@@@a,bbbbbbbbbbbbbbbbbbbbb
    // bbbbbbbba@@@@@@@@@@@@@@@@@@@@@' . `@@@@@@@@@@@@@@@@@@@@@@@@bbbbbbbbbbbbbbbbbbbbb
    // bbbbbbbb;`@@@@@@@@@@@@@@@@@@'   .   `@@@@@@@@@@@@@@@@@@@@@';bbbbbbbbbbbbbbbbbbbb
    // bbbbbbbb;@@@`@@@@@@@@@@@@@'     .     `@@@@@@@@@@@@@@@@'@@@;bbbbbbbbbbbbbbbbbbbb
    // bbbbbbbb;@@@;,.aaaaaaaaaa       .       aaaaa,,aaaaaaa,;@@@;bbbbbbbbbbbbbbbbbbbb
    // bbbbbbbb;;@;;;;@@@@@@@@;@      @.@      ;@@@;;;@@@@@@;;;;@@;bbbbbbbbbbbbbbbbbbbb
    // bbbbbbbb;;;;;;;@@@@;@@;;@    @@ . @@    ;;@;;;;@@;@@@;;;;;;;bbbbbbbbbbbbbbbbbbbb
    // bbbbbbbb;;;;;;;;@@;;;;;;;  @@   .   @@  ;;;;;;;;;;;@@;;;;@;;bbbbbbbbbbbbbbbbbbbb
    // bbbbbbbb;;;;;;;;;;;;;;;;;@@     .     @@;;;;;;;;;;;;;;;;@@@;bbbbbbbbbbbbbbbbbbbb
    // bbbbbbbb;;;;;;;;@;;;;;;;;       .       ;;;;;;;;;;;;;;;;@@;;bbbbbbbbbbbbbbbbbbbb
    // bbbbbbbb;;;;;;;@@;;;;;;;;     ,bbb,     ;;;;;;;;;;;;;;;;;;;;bbbbbbbbbbbbbbbbbbbb
    // bbbbbbbb;;;;;;;@@;;;;;;;;   ,bbbbbbb,   ;;;;;;;;;;;;;;;;;;;;bbbbbbbbbbbbbbbbbbbb
    // bbbbbbbb`;;;;;;;;;;;;;;;;  bbbbbbbbbbb  ;;;;;;;;;;;;;;;;;;;'bbbbbbbbbbbbbbbbbbbb
    // bbbbbbbbbbbb`;;;;;;;;;;;;,bbbbbbbbbbbbb,;;;;;;;;;;;;;;;'bbbbbbbbbbbbbbbbbbbbbbbb
    // bbbbbbbbbbbbbbbbbb,,,,,,,bbbbbbbbbbbbbbb,,,,,,,bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
    // bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"

    let uncolored_background =
        "wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

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
                'b' => colored_background.push_str(&format!(
                    "{}{} {}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(130)),
                    termion::color::Bg(termion::color::Reset),
                )),
                _ => unreachable!(),
            }
        }
    }
    let uncolored_cake = "                                         



                ,a@@@@@@@@@@@@@@@@@@@@@@@@@@@@a@@@a.
           .,a@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@a,.
         ,a@@@@@@@@@@@@@@@@@@@@@.@@@@@@@@@@@@@@@@@@@@@@@@a,                     
        a@@@@@@@@@@@@@@@@@@@@@'d.d`@@@@@@@@@@@@@@@@@@@@@@@@                     
        ;`@@@@@@@@@@@@@@@@@@'ddd.ddd`@@@@@@@@@@@@@@@@@@@@@';                    
        ;@@@`@@@@@@@@@@@@@'ddddd.ddddd`@@@@@@@@@@@@@@@@'@@@;                    
        ;@@@;,.aaaaaaaaaaddddddd.dddddddaaaaa,,aaaaaaa,;@@@;                    
        ;;@;;;;@@@@@@@@;@dddddd@.@dddddd;@@@;;;@@@@@@;;;;@@;                    
        ;;;;;;;@@@@;@@;;@dddd@@d.d@@dddd;;@;;;;@@;@@@;;;;;;;                    
        ;;;;;;;;@@;;;;;;;dd@@ddd.ddd@@dd;;;;;;;;;;;@@;;;;@;;                    
        ;;;;;;;;;;;;;;;;;@@ddddd.ddddd@@;;;;;;;;;;;;;;;;@@@;                    
        ;;;;;;;;@;;;;;;;;ddddddd.ddddddd;;;;;;;;;;;;;;;;@@;;                    
        ;;;;;;;@@;;;;;;;;ddddd,   ,ddddd;;;;;;;;;;;;;;;;;;;;                    
        ;;;;;;;@@;;;;;;;;ddd,       ,ddd;;;;;;;;;;;;;;;;;;;;                    
        `;;;;;;;;;;;;;;;;dd           dd;;;;;;;;;;;;;;;;;;;'                    
            `;;;;;;;;;;;;,             ,;;;;;;;;;;;;;;;'                        
                  ,,,,,,,               ,,,,,,,";

    let mut colored_cake = String::new();
    for (vertical_offset, line) in uncolored_cake.lines().enumerate() {
        for (horizontal_offset, char) in line.chars().enumerate() {
            match char {
                ' ' => continue,
                'd' => colored_cake.push_str(&format!(
                    "{}{} {}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(52)),
                    termion::color::Bg(termion::color::Reset),
                )),
                c @ ('@' | 'a') => colored_cake.push_str(&format!(
                    "{}{}{}{}{}{}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(231)),
                    termion::color::Fg(termion::color::AnsiValue(253)),
                    c,
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset),
                )),
                c @ (';' | ',' | '.' | '`' | '\'') => colored_cake.push_str(&format!(
                    "{}{}{}{}{}{}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(224)),
                    termion::color::Fg(termion::color::AnsiValue(249)),
                    c,
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset),
                )),
                _ => unreachable!(),
            }
        }
    }

    let uncolored_candles = "
                          (              
                  )      (*)                    (
                 (*)      )                    (*)
                  (      |~|                    (
                 |~|     |r|                   |~| 
                 |r|     |r|                   |r|       
                 |r|                           |r|";

    let mut colored_candles = String::new();
    for (vertical_offset, line) in uncolored_candles.lines().enumerate() {
        for (horizontal_offset, char) in line.chars().enumerate() {
            match char {
                ' ' => continue,
                c @ ('|' | '~') => colored_candles.push_str(&format!(
                    "{}{}{}{}{}{}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(160)),
                    termion::color::Fg(termion::color::AnsiValue(88)),
                    c,
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset),
                )),
                'r' => colored_candles.push_str(&format!(
                    "{}{} {}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(160)),
                    termion::color::Bg(termion::color::Reset),
                )),
                c @ ('(' | ')' | '*') => colored_candles.push_str(&format!(
                    "{}{}{}{}{}{}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(termion::color::AnsiValue(172)),
                    termion::color::Fg(termion::color::AnsiValue(214)),
                    c,
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset),
                )),
                c => panic!("Reached unreachable: {}", c),
            }
        }
    }

    let birthday_wishes = format!(
        "{}{}{}Happy Birthday{}{}",
        termion::cursor::Goto(64, 2),
        termion::color::Bg(termion::color::AnsiValue(250)),
        termion::color::Fg(termion::color::AnsiValue(197)),
        termion::color::Bg(termion::color::Reset),
        termion::color::Fg(termion::color::Reset)
    );

    // draw cake
    write!(
        stdout,
        "{}{}{}{}{}{}",
        termion::clear::All,
        colored_background,
        colored_cake,
        colored_candles,
        birthday_wishes,
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
}
