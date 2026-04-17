use std::io::{self, Write, stdout};
use std::fs;
use std::env;

fn main() -> io::Result<()> {
    
    main_input()

}


        //Receiving Result from enum
fn main_input() -> io::Result<()> {
    
    println!("Welcome to St Organiser\n\n");
    println!("[1] Quit | [2] Directory | [3] Help |\n");
    stdout().flush()?;

    loop {
        let mut input: String = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        let command: String = input.trim().to_lowercase();

        let choice = match command.as_str() {
            "1" => break,
            "2" => MenuChoice::Dir,
            "3" => MenuChoice::Help,
            _ => {
                println!("Unknown Command");
                continue;
            }
        };
        
        choice.execute()?;
    }
    
    Ok(())

                        //Receiving First Input to enter different menus//
                        //Such as Directory Home Help//
}

                        //Choice Selection and Display Messages//
enum MenuChoice {
    Dir,
    Help,
    Home,
}


                        //Methods to handle Menu Choices
impl MenuChoice {
    
    fn execute(&self) -> io::Result<()> {
        
        match self {
            MenuChoice::Dir => self.dir(),
            MenuChoice::Help => self.help(),
            MenuChoice::Home => self.home(),
        }
    }

//
    fn dir(&self) -> io::Result<()> {
        
        println!("Please enter a Directory\nUsing Full Expression...\n\n");
        stdout().flush()?;
        
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)?;
        
        let path = input.trim().to_lowercase();
        
        env::set_current_dir(&path)?;
        self.dir_choice(path.to_string())
    }
//
    fn dir_choice(&self, selection: String) -> io::Result<()> {
        
        println!("Current Directory: {}", selection);
        stdout().flush()?;

        self.folder_contents(selection)
    }
//

    fn help(&self) -> io::Result<()> {

        println!("Welcome to the Help Screen!\n");
        stdout().flush()

    }
//
    fn home(&self) -> io::Result<()> {
        
        println!("Home\n\n");
        stdout().flush()

    }
    fn folder_contents(&self, folder: String) -> io::Result<()> {
        let entries = fs::read_dir(folder)?;
        for entry in entries {
            if let Ok(entry) = entry {
                println!("Entry Path: {:?}", entry.path());
            }
        }
        stdout().flush()
    }
}

