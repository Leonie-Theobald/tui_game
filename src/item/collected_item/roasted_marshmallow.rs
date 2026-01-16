use crate::item::{Item, collected_item::Collectable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoastedMarshmallow;

impl Item for RoastedMarshmallow {
    fn get_name(&self) -> &'static str {
        "Roasted Marshmallow"
    }

    fn get_description(&self) -> &'static str {
        "A roasted, crispy marshmallow"
    }

    fn get_detail_view(&self) -> String {
        let plain_marshmallow = String::from(
            "
                                                          \r
                             _____________________        \r
                         ___/.....................\\___   \r
                        /.............................\\  \r
                       |\\_..........................._/| \r
                       |..\\_________________________/..| \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                        \\_..........................._/  \r
                          \\_________________________/    \r
                                                         ",
        );

        plain_marshmallow
            .chars()
            .fold(String::new(), |mut brown_marshmallow, c| {
                if c.is_whitespace() {
                    brown_marshmallow.push(c);
                } else {
                    brown_marshmallow.push_str(&format!(
                        "{}{}{}{}{}",
                        termion::color::Bg(termion::color::AnsiValue(130)),
                        termion::color::Fg(termion::color::Black),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset)
                    ))
                }
                brown_marshmallow
            })
            .replace(".", " ")
    }
}

impl Collectable for RoastedMarshmallow {
    fn get_symbol(&self) -> String {
        String::from("🟫")
    }
}
