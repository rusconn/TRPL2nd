use std::fmt::Display;

pub fn print_list<T: Display>(header: &str, items: &[T]) {
    println!("{}:", header);

    for item in items {
        println!("\t{}", item);
    }
}

pub fn eprint_list<T: Display>(header: &str, items: &[T]) {
    eprintln!("{}:", header);

    for item in items {
        eprintln!("\t{}", item);
    }
}
