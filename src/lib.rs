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
            Some(0) => print_items_from_list(& todo),
            Some(1) => add_item_to_list(&mut todo),
            Some(2) => toggle_item_on_list(&mut todo),
            Some(3) =>
            {
                println!("Saved, goodbye!");
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

    // necessary since Input starts enumeration
    // at 1, not 0
    index -= 1;

    list.toggle(index).unwrap_or_else(|err| {
        println!("Error while toggling: {err}");
    });
}
