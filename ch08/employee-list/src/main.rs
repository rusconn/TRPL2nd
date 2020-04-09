use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io;

use employee_list::print::eprint_list;
use employee_list::{run, Command, Employees};

fn main() {
    eprint_commands();

    let s = RandomState::new();
    let mut employees: Employees = HashMap::with_hasher(s);

    loop {
        eprint!("> ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "End" {
            break;
        }

        match Command::try_from(input) {
            Ok(command) => run(command, &mut employees),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn eprint_commands() {
    let commands = [
        "Add <PERSON> to <DEPARTMENT>",
        "List <DEPARTMENT>",
        "List",
        "End",
    ];

    eprint_list("Commands", &commands);
}
