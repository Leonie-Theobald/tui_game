pub mod digit;
pub mod password;

use inquire::CustomType;
use serde::{Deserialize, Serialize};

use crate::item::{
    collected_item::CollectedItems,
    interaction::{
        Interactable, InteractionStatus,
        terminal_interaction::password::{PASSWORD_LENGTH, Password},
    },
    stationed_item::Stationary,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TerminalInteraction {
    EnterDigits(EnterDigits),
}

impl TerminalInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            TerminalInteraction::EnterDigits(inner) => inner,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterDigits;
impl Interactable for EnterDigits {
    fn get_prompt(&self) -> String {
        String::from("Enter Password")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        _collected_items: &mut CollectedItems,
    ) -> String {
        match linked_stationary_item.get_interaction_status() {
            InteractionStatus::Open => {
                let result =
                    CustomType::<Password>::new(&format!("Type in {:?} digits:", PASSWORD_LENGTH))
                        .with_error_message("Please type a valid number")
                        .with_help_message("E.g. 123456")
                        .prompt();

                let correct_password = Password::get_correct();

                match result {
                    Ok(number) if number == correct_password => {
                        linked_stationary_item.set_interaction_status_to_finished();
                        format!("{} is the correct password", number)
                    }
                    Ok(number) => format!("{} is not the correct password", number),

                    Err(_error) => "Could not get password".to_owned(),
                }
            }
            InteractionStatus::Finished => String::from("Correct password already entered."),
        }
    }
}
