use accountcmd::AccountCmd;
use dynmenucomp::DynMenuStruct;
use dynmenucomp::ExecutableMenu;
use dynmenucomp::Menu;
use dynmenucomp::MenuOption;
use std::error::Error;
use std::{collections::HashMap, process};
use tinyexchange::Command;
mod accountcmd;
mod dynmenucomp;

// Menu options constants
static ACCOUNT: &str = "account";
static TRADE: &str = "trade";
static MARKET: &str = "market";

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
    let mut menu_component = dynmenucomp::initialize();

    // Populate account, trade, and mkt menus
    populate_menus(&mut menu_component);

    // Calls the config run()
    if let Err(e) = run(command, menu_component) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// Initialize menus
fn populate_menus(_dynMenu: &mut DynMenuStruct) {
    // Populate Account Menus
    _dynMenu
        .menu_list
        .insert(MenuOption::Account, build_account_menu());

    // Populate Market Menus
    _dynMenu
        .menu_list
        .insert(MenuOption::Market, build_market_menu());

    // Populate Market Menus
    _dynMenu
        .menu_list
        .insert(MenuOption::Trade, build_trade_menu());
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

/* Parse domains and commands
 * @domains: account, trade, market
 * @commands: account - deposit, withdraw, balance
 *            market - assets
 *            trade - buy, sell, portfolio
 */
pub fn run(_command: Command, _dynMenu: dynmenucomp::DynMenuStruct) -> Result<(), Box<dyn Error>> {
    // Prints the command and key
    //println!("Domain: {0} - Key: {1}", command.domain, command.key);

    // Find the menu level entry
    let key = str_to_enum(_command.domain);
    let menu = _dynMenu.menu_list.get(&key).unwrap();

    // Unwrap the Menu inside the Option<> provided by Rust
    // and access the hashmap of submenus
    let domain = str_to_enum(_command.key);

    // TO-DO: implement iterator
    // Executes the menu option handler
    menu.exec_menus
        .get(&domain)
        .unwrap()
        .execute(domain, Vec::new());

    Ok(())
}

// Parse a string argument into an Enum variant
pub fn str_to_enum(_input: String) -> MenuOption {
    let mut option: MenuOption = MenuOption::Account;

    // Compare pre-defined options against the user input
    if _input == ACCOUNT {
        option = MenuOption::Account
    } else if (_input == MARKET) {
        option = MenuOption::Market
    } else if (_input == TRADE) {
        option = MenuOption::Trade
    }

    return option;
}
