// Copyright (C) 2024 Riley Rager, DragonShadow9
// This file assemplies the program and makes it usable and sets up the wizerd for the user 
/*
commands:
init : sets up the bin folder and the wizerd and starts it
wiz : starts the wizerd to install programs and other cool things to the termianl
uninstall : whipes the program's bin *don't use this to del programs use the wizerd to do that*
*/

mod macros;
use colored::Colorize;
use std::fs;
use std::io;
use macros::*;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        errormsg("No command given do 'help' for a list of commands");
    } else {
        systemCommand("ls", false);
    }
}