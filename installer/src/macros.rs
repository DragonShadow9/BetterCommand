// copyright (C) 2024 Riley Rager, DragonShadow9

use colored::Colorize;

pub fn errormsg(message: &str) {
    println!("{} {}","Error: ".red(),message.red());
}


pub fn systemCommand(command: &str, output: bool) {
    let result = std::process::Command::new(command)
        .output()
        .expect("failed to execute process");

    if output == true {
        println!("{}", String::from_utf8_lossy(&result.stdout));
    }
    
}