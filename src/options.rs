use std::process::exit;

#[derive(Clone)]
pub enum Command {
    None,
    Clone,
    Checkout,
    Pull,
    Push,
}

pub struct Options {
    command: Command,
    repo: String,
    branch: String,
    help: bool,
}

impl Options {
    fn new() -> Options {
        Options {
            command: Command::None,
            repo: String::new(),
            branch: String::new(),
            help: false,
        }
    }

    pub fn get_command(&self) -> Command {
        self.command.clone()
    }

    pub fn get_help(&self) -> bool {
        self.help.clone()
    }

    pub fn get_repo(&self) -> String {
        self.repo.clone()
    }

    pub fn get_branch(&self) -> String {
        self.branch.clone()
    }
}

pub fn parse(args: &Vec<String>) -> Options {
    let mut options = Options::new();

    // first ( 0 ) arguments is the name of the program
    let mut i: usize = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" => options.help = true,
            "clone" => {
                options.command = Command::Clone;

                i = i + 1;
                if i >= args.len() {
                    println!("bog: 'bog clone' requires REPO argument.");
                    println!("Usage: bog clone REPO");
                    exit(1);
                }

                options.repo = args[i].clone();
            },
            "checkout" => {
                options.command = Command::Checkout;

                i = i + 1;
                if i >= args.len() {
                    println!("bog: 'bog checkout' requires BRANCH argument.");
                    println!("Usage: bog checkout BRANCH");
                    exit(1);
                }

                options.branch = args[i].clone();
            },
            "pull" => {
                options.command = Command::Pull;
            },
            "push" => {
                options.command = Command::Push;
            },

            &_ => {
                println!("bog: '{}' is not valid command", args[i]);
                exit(1);
            }
        }
        i = i + 1;
    }

    options
}