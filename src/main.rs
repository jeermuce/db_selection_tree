use clap::{Arg, Command};
use dialoguer::{theme::ColorfulTheme, Confirm};

fn main() {
    let matches = Command::new("db_selection_tree")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Helps to select SQL or NoSQL approach")
        .get_matches();

    let big_data = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is the data big?")
        .interact()
        .unwrap();

    let data_integrity = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is data integrity important?")
        .interact()
        .unwrap();

    let low_latency = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is low latency required?")
        .interact()
        .unwrap();

    let flexible_schema = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is a flexible schema needed?")
        .interact()
        .unwrap();

    let repeatable_access_pattern = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is there a repeatable access pattern?")
        .interact()
        .unwrap();

    let partition_and_sort_key = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is there a partition and sort key that cover needed queries?")
        .interact()
        .unwrap();

    if big_data {
        println!("NoSQL");
    } else if data_integrity {
        println!("SQL");
    } else if low_latency {
        println!("NoSQL");
    } else if flexible_schema {
        println!("NoSQL");
    } else if repeatable_access_pattern {
        println!("SQL");
    } else if partition_and_sort_key {
        println!("SQL");
    } else {
        println!("No clear recommendation, use whichever you are more comfortable with");
    }
}
