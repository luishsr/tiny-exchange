use accountcmd::AccountCmd;
use dynmenu::*;
use std::{collections::HashMap, process};
use tinyexchange::Command;
mod accountcmd;
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
    let mut menu_component: DynMenu = dynmenu::initialize();

    // Populate account, trade, and mkt menus
    populate_menus(&mut menu_component);

    // Calls the config run()
    if let Err(e) = tinyexchange::run(command) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// Initialize menus
fn populate_menus(_dynMenu: &mut DynMenu) {
    // Populate Account Menus
    _dynMenu.menu_list.insert(1, build_account_menu());

    // Populate Market Menus
    _dynMenu.menu_list.insert(2, build_market_menu());

    // Populate Market Menus
    _dynMenu.menu_list.insert(3, build_trade_menu());
}

// Build Account-related Menu items
fn build_account_menu() -> Menu {
    // The list of submenus (ExecutableMenu) under the Account Menu
    let mut exec_menus: HashMap<MenuOption, Box<dyn ExecutableMenu>> = HashMap::new();

    // The root Account menu
    let account_cmd = AccountCmd {
        id: MenuOption::Account,
    };

    // Add the ExecutableMenu to the list
    exec_menus.insert(MenuOption::Account, Box::new(account_cmd));

    // Return the Menu struct
    return Menu {
        id: MenuOption::Account,
        exec_menus: exec_menus,
    };
}

// Build Market-related Menu items
fn build_market_menu() -> Menu {
    // The list of submenus (ExecutableMenu) under the Account Menu
    let mut exec_menus: HashMap<MenuOption, Box<dyn ExecutableMenu>> = HashMap::new();

    // The root Account menu
    let market_cmd = AccountCmd {
        id: MenuOption::Market,
    };

    // Add the ExecutableMenu to the list
    exec_menus.insert(MenuOption::Market, Box::new(market_cmd));

    // Return the Menu struct
    return Menu {
        id: MenuOption::Market,
        exec_menus: exec_menus,
    };
}

// Build Trade-related Menu items
fn build_trade_menu() -> Menu {
    // The list of submenus (ExecutableMenu) under the Account Menu
    let mut exec_menus: HashMap<MenuOption, Box<dyn ExecutableMenu>> = HashMap::new();

    // The root Account menu
    let trade_cmd = AccountCmd {
        id: MenuOption::Trade,
    };

    // Add the ExecutableMenu to the list
    exec_menus.insert(MenuOption::Trade, Box::new(trade_cmd));

    // Return the Menu struct
    return Menu {
        id: MenuOption::Trade,
        exec_menus: exec_menus,
    };
}
