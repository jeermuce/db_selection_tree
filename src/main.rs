use clap::Command;
use dialoguer::{theme::ColorfulTheme, Confirm};

fn main() {
    Command::new("db_selection_tree")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Helps to select SQL or NoSQL approach")
        .get_matches();

    let big_data = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is the data big?")
        .interact()
        .unwrap();

    if big_data {
        println!("NoSQL (recommended for big data)");
        return;
    }

    let data_integrity = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is data integrity important?")
        .interact()
        .unwrap();

    if data_integrity {
        println!("SQL (recommended for ACID compliance and data integrity)");
        return;
    }

    let low_latency = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is low latency required?")
        .interact()
        .unwrap();

    if low_latency {
        println!("NoSQL (recommended for low latency)");
        return;
    }

    let flexible_schema = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is a flexible schema needed?")
        .interact()
        .unwrap();

    if flexible_schema {
        println!("NoSQL (recommended for flexible schema requirements)");
        return;
    }

    // Data Access Questions
    let repeatable_access_pattern = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is there a repeatable access pattern?")
        .interact()
        .unwrap();

    if !repeatable_access_pattern {
        println!("SQL (best suited for varied access patterns)");
        return;
    }

    let partition_and_sort_key = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is there a partition and sort key that covers needed queries?")
        .interact()
        .unwrap();

    if partition_and_sort_key {
        println!("NoSQL (works well with partitioned and sorted data access)");
    } else {
        println!("SQL (better for non-partitioned data access)");
    }
}
