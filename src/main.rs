use dynmenu::*;
use std::{collections::HashMap, error::Error, process, str::Bytes};
use tinyexchange::Command;
mod dynmenu;

// The entry point of a CLI-based exchange
fn main() {
    // Load arguments from the program call into a variable
    let args: Vec<String> = std::env::args().collect();

    // Validate initial arguments
    let command = Command::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Initialize the Menu component
    let menu_component: DynMenu = dynmenu::initialize();

    // Calls the config run()
    if let Err(e) = tinyexchange::run(command) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// Initialize menus
fn populate_menus(_dynMenu: &DynMenu) {
    // Initializes Account Menus
    //_dynMenu.menu_list.insert(index, element);
}

// Build Account-related Menu items
fn build_account_menu() -> Menu {
    // Start with Account submenus (ExecutableMenu trait)
    

    // Return a new Menu struct
    return Menu {
        id: 1,
        name: String::from("New"),
        exec_menus: HashMap::new(),
    };
}
