mod dynmenu;
use dynmenu::*;
use std::error::Error;

// Represents the arguments
pub struct Command {
    pub domain: String,
    pub key: String,
}

// Menu options constants
static ACCOUNT: &str = "account";
static TRADE: &str = "trade";
static MARKET: &str = "market";

// Initialize a Command struct
impl Command {
    //TO-DO: add error handling
    pub fn build(args: &[String]) -> Result<Command, &'static str> {
        // Check if the minimum number of parameters have been provided
        if args.len() < 3 {
            panic!("Not enough parameters provided!");
        }

        // Return variables
        let domain = args[1].clone();
        let key = args[2].clone();

        // Return
        Ok(Command { domain, key })
    }
}

/* Parse domains and commands
 * @domains: account, trade, market
 * @commands: account - deposit, withdraw, balance
 *            market - assets
 *            trade - buy, sell, portfolio
 */
pub fn run(_command: Command, _dynMenu: &mut DynMenu) -> Result<(), Box<dyn Error>> {
    // Prints the command and key
    //println!("Domain: {0} - Key: {1}", command.domain, command.key);

    // Find the menu level entry
    let key = str_to_enum(_command.domain);
    let menu: Option<&Menu> = _dynMenu.menu_list.get(&key);
    
    // Unwrap the Menu inside the Option<> provided by Rust
    // and access the hashmap of submenus
     menu?.exec_menus.get(command.key);

    // Loads the submenus within the menu level entry above
    let executable_menu = menu.;

    Ok(())
}

// Parse a string argument into an Enum variant
pub fn str_to_enum(_input: String) -> MenuOption {
    let option: MenuOption;

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
