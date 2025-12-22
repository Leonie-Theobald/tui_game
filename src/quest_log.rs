// impl Drawable for QuestLog {
//     fn draw(&self) -> String {
//         if !self.visible {
//             return String::new();
//         }
//
//         let unsolved_quests = self.get_unsolved_quests();
//
//         // format each quest line and find out longest quest line
//         let mut length_longest_quest_line = 0;
//         let mut joined_quests = String::new();
//         for (index, quest) in unsolved_quests.iter().enumerate() {
//             let line = format!(
//                 "{}{}{}{}",
//                 termion::cursor::Goto(4, 3 + index as u16),
//                 color::Bg(color::Blue),
//                 quest,
//                 color::Bg(color::Reset),
//             );
//             joined_quests.push_str(&line);
//
//             let description = method_on_inner_quest!(quest, get_description);
//             if description.len() > length_longest_quest_line {
//                 length_longest_quest_line = description.len();
//             }
//         }
//
//         // format background depending on number of quests and the longest quest line
//         // line length depends on quests length
//         let background_line = format!(
//             "{}{}{}",
//             color::Bg(color::Blue),
//             " ".repeat(length_longest_quest_line + 4),
//             color::Bg(color::Reset),
//         );
//
//         // height depends on number of quests
//         let mut background = String::new();
//         for index in 0..unsolved_quests.len() + 2 {
//             background.push_str(&format!(
//                 "{}{}",
//                 termion::cursor::Goto(2, 2 + index as u16),
//                 background_line
//             ))
//         }
//
//         // merge background and written quests
//         format!("{}{}", background, joined_quests)
//     }
// }
