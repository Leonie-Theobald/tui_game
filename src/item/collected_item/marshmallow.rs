use crate::item::{Item, collected_item::Collectable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Marshmallow;

impl Item for Marshmallow {
    fn get_name(&self) -> &'static str {
        "Marshmallow"
    }

    fn get_description(&self) -> &'static str {
        "An unroasted, squishy marshmallow"
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
            .fold(String::new(), |mut white_marshmallow, c| {
                if c.is_whitespace() {
                    white_marshmallow.push(c);
                } else {
                    white_marshmallow.push_str(&format!(
                        "{}{}{}{}{}",
                        termion::color::Bg(termion::color::White),
                        termion::color::Fg(termion::color::LightBlack),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset)
                    ))
                }
                white_marshmallow
            })
            .replace(".", " ")
    }
}

impl Collectable for Marshmallow {
    fn get_symbol(&self) -> String {
        String::from("🖱️")
    }
}
