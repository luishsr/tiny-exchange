use dynmenu::DynMenu;
use std::{process, str::Bytes};
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
    let menu_component: DynMenu = initialize_menus();

    // Calls the config run()
    if let Err(e) = tinyexchange::run(command) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// Initialize menus
fn initialize_menus() -> DynMenu {
    // Instantiate a Dynamic Menu component
    let menuComponent = dynmenu::initialize();

    // Initializes Account Menus
    //menuComponent.
}
