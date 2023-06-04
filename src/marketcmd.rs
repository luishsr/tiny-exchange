use std::error::Error;

use crate::dynmenucomp::{ExecutableMenu, MenuOption};

pub struct MarketCmd {
    pub id: MenuOption,
}

impl ExecutableMenu for MarketCmd {
    fn execute(&self, _type: MenuOption, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        println!("AccountMarket** executable menu - execute()");
        return Ok(());
    }
}
