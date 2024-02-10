/* I want to create a to-do list on the command line

    initial features
    1. add items to the list - done
    2. mark items as complete - done
    3. unmark items as complete - done

    interactive features (interactive mode):
    1. print menu for user - done
    2. option: print list - done
    3. option: add item to list - done
    3. option: mark item to list - done

    saving features
    1. save list to file
    2. read list from file
*/

use std::process;
use to_do::ToDo;

fn main() {
    let interactive = true;
    let savefile = "save.txt";

    let mut todo = ToDo::new();

    if interactive {
        to_do::interactive_run(&mut todo).unwrap_or_else(|err| {
            eprintln!("Problem during interactive run: {err}");
            process::exit(1);
        });
    };

    println!("Saving...");
    to_do::save(savefile, &todo).unwrap_or_else(|err| {
        eprintln!("Problem during save: {err}");
        process::exit(2);
    });
    println!("Saved, goodbye!");
}
