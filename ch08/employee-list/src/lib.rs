pub mod print;

use std::collections::HashMap;
use std::convert::TryFrom;

use print::print_list;

pub type Employees = HashMap<String, Vec<String>>;

pub enum Command<'a> {
    Add(&'a str, &'a str),
    List(&'a str),
    ListAll,
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = String;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let mut words = input.split_whitespace();

        match words.next() {
            Some("Add") => {
                let person = match words.next() {
                    Some(person) => person,
                    _ => return Err(String::from("arg PERSON not found.")),
                };

                match words.next() {
                    Some("to") => (),
                    Some(x) => return Err(format!("Should be \"to\": {}", x)),
                    _ => return Err(String::from("\"to\" not found.")),
                }

                let department = match words.next() {
                    Some(department) => department,
                    _ => return Err(String::from("arg DEPARTMENT not found.")),
                };

                Ok(Self::Add(person, department))
            }
            Some("List") => {
                if let Some(department) = words.next() {
                    Ok(Self::List(department))
                } else {
                    Ok(Self::ListAll)
                }
            }
            Some(x) => Err(format!("Unknown command: {}", x)),
            _ => Err(String::from("Command not found.")),
        }
    }
}

pub fn run(command: Command, employees: &mut Employees) {
    match command {
        Command::Add(person, department) => add(person, department, employees),
        Command::List(department) => list(department, employees),
        Command::ListAll => list_all(employees),
    }
}

fn add(person: &str, department: &str, employees: &mut Employees) {
    employees
        .entry(department.to_string())
        .or_default()
        .push(person.to_string());
}

fn list(department: &str, employees: &Employees) {
    if let Some(people) = employees.get(department) {
        print_list(department, &people);
    } else {
        eprintln!("Department {} not found.", department);
    }
}

fn list_all(employees: &Employees) {
    let mut employees: Vec<_> = employees.iter().collect();
    employees.sort_by_key(|&(department, _)| department);

    for (department, people) in employees {
        print_list(department, people);
    }
}
