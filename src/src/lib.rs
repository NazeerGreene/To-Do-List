use std::fs;
use std::io::Error;
use dialoguer::{Input, Select};

mod todo;
pub use todo::{Item, ToDo};

pub fn interactive_run(list: &mut ToDo) -> Result<(), &'static str> {

    let interactive_menu = [
    "Print list",
    "Add item to list",
    "Mark item on list",
    "Save and Quit"
    ];

    loop {
        println!("\nTo Do List");

        let menu_item = Select::new()
            .with_prompt("Select [tab], choose [enter], or quit [q/ESC]")
            .default(0)
            .items(&interactive_menu)
            .interact_opt()
            .unwrap();

        match menu_item {
            Some(0) => print_items_from_list(& list),
            Some(1) => add_item_to_list(list),
            Some(2) => toggle_item_on_list(list),
            Some(3) =>
            {
                println!("Will save...");
                break;
            },
            Some(_) => return Err("Invalid selection from menu!"),
            None =>
            {
                println!("Goodbye!");
                break;
            },
        } // match
    };

    Ok(())
}

fn print_items_from_list(list: & ToDo) {
    if list.len() < 1 {
        println!("\nNo items in list!\n");
        return;
    }

    println!();
    for (i, item) in list.items.iter().enumerate() {
        let complete = if item.is_complete { 'X' } else { ' ' };
        println!("{}. [{}] {}", i+1, complete, item.descr);
    }
    println!();
}

fn add_item_to_list(list: &mut ToDo) {
    let new_item: String = Input::new()
        .with_prompt("\nAdd to list")
        .allow_empty(true)
        .interact()
        .unwrap();

    if !new_item.is_empty() {
        list.add(&new_item);
    }
}

fn toggle_item_on_list(list: &mut ToDo) {

    if list.len() < 1 {
        println!("\nNo items in list!");
        return;
    }

    print_items_from_list(&list);

    let index: String = Input::new()
        .with_prompt("Which item?")
        .allow_empty(true)
        .interact()
        .unwrap();

    if index.is_empty() { return; };

    let mut index: usize = match index.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Input must be an integer!");
            return;
        },
    };

    if index < 1 || index > list.len() {
        println!("Select from the available options!");
        return;
    }

    // necessary since Input starts enumeration
    // at 1, not 0
    index -= 1;

    list.toggle(index).unwrap_or_else(|err| {
        println!("Error while toggling: {err}");
    });
}

pub fn save(file: &str, list: &ToDo) -> Result<(), Error> {
    let mut data_to_write = Vec::with_capacity(list.len());

    for item in list.items.iter() {
        let completed = if item.is_complete {'1'} else {'0'};

        data_to_write.push(
            format!("{}:{}", completed, item.descr)
        );
    };

    //println!("{:?}", data_to_write);

    fs::write(file, data_to_write.join("\n"))
}

pub fn load(file: &str) -> Result<ToDo, Error> {
    Ok(ToDo::new())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_list() {
        let mut list = ToDo::new();

        list.add("hello world");

        list.add("goodbye world");
        list.toggle(1).unwrap();

        save("save.txt", &list).unwrap();
    }
}
