/*
 * @dev The dynmenu provides a dynamic list of
 * nested commands (i.e., menu options) that takes a list
 * of arguments and lookup the associated interface implementation
 * to handle it.
 *
 * Luis Soares - 2023
 */

// Holds a list of Menu objects at a component level
struct DynMenu {
    menu_list: HashMap,
    id_counter: u32,
}

// Implements the DynMenu component
pub impl DynMenu {
    pub fn initialize() {
        //TO-DO
    }

    // Registers a new menu option
    pub fn add_menu(&_menu: Menu) -> Result {
        //TO-DO: check for duplicates
        menu_list.add(_name, _subMenus);
    }
}

// Represents a menu option
pub struct Menu {
    id: u32,
    name: String,
    exec_menus: HashMap,
}

// Represents an executable menu option
pub struct SubMenu {
    id: u32,
    name: String,
    function: ExecutableMenu,
}

// Defines a trait for executing a function within a menu option
pub trait ExecutableMenu {
    pub fn execute() -> Result;
}
