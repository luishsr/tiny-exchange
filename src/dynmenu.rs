use std::collections::HashMap;

/*
 * @dev The dynmenu provides a dynamic list of
 * nested commands (i.e., menu options) that takes a list
 * of arguments and lookup the associated interface implementation
 * to handle it.
 *
 * Luis Soares - 2023
 */

// Holds a list of Menu objects at a component level
pub struct DynMenu {
    pub menu_list: Vec<Menu>,
    pub id_counter: u32,
}

// Menu Enum
enum MenuOption {
    ACC_CREATE,
    ACC_BALANCE,
    ACC_DEPOSIT,
    ACC_WITHDRAW,
    MKT_ASSETS,
    TRD_BUY,
    TRD_SELL,
    TRADE,
    MARKET,
    ACCOUNT,
}

// Implements the DynMenu component
pub fn initialize() -> DynMenu {
    DynMenu {
        menu_list: Vec::new(),
        id_counter: 0,
    }
}

// Registers a new menu option
pub fn add_menu(_menu: &Menu) {}

// Represents a menu option
pub struct Menu {
    pub id: u32,
    pub name: String,
    pub exec_menus: HashMap<String, Box<dyn ExecutableMenu>>,
}

// Defines a trait for executing a function within a menu option
pub trait ExecutableMenu {
    fn execute(&self, _type: MenuOption, args: Vec<String>);
}
