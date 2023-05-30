use std::error::Error;

use crate::dynmenu::{ExecutableMenu, MenuOption};

pub struct AccountCmd {
    pub id: MenuOption,
}

impl ExecutableMenu for AccountCmd {
    fn execute(&self, _type: MenuOption, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        return Ok(());
    }
}
