mod dynmenucomp;

// Represents the arguments
pub struct Command {
    pub domain: String,
    pub key: String,
}

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
