use std::{collections::HashMap, error::Error};

/*
 * @dev The dynmenu provides a dynamic list of
 * nested commands (i.e., menu options) that takes a list
 * of arguments and lookup the associated interface implementation
 * to handle it.
 *
 * Luis Soares - 2023
 */

// Holds a list of Menu objects at a component level
pub struct DynMenuStruct {
    pub menu_list: HashMap<MenuOption, Menu>,
    pub id_counter: u32,
}

// Menu Enum
#[derive(Eq, Hash, PartialEq)]
pub enum MenuOption {
    AccCreate,
    AccBalance,
    AccDeposit,
    AccWithdraw,
    MktAssets,
    TradeBuy,
    TradeSell,
    Trade,
    Market,
    Account,
}

// Implements the DynMenu component
pub fn initialize() -> DynMenuStruct {
    DynMenuStruct {
        menu_list: HashMap::new(),
        id_counter: 0,
    }
}

// Registers a new menu option
pub fn add_menu(_menu: &Menu) {}

// Represents a menu option
pub struct Menu {
    pub id: MenuOption,
    pub exec_menus: HashMap<MenuOption, Box<dyn ExecutableMenu>>,
}

// Defines a trait for executing a function within a menu option
pub trait ExecutableMenu {
    fn execute(&self, _type: MenuOption, args: Vec<String>) -> Result<(), Box<(dyn Error)>>;
}
