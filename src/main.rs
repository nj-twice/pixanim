use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.iter().count() - 1 {
        0 => panic!("No argument supplied!"),
        1 => {}
        _ => panic!("More than one argument supplied!"),
    }

    let action = &args[1];

    match action.as_str() {
        "make" => make_spritesheet(),
        "play" => visualize_animation(),
        "help" => show_help(),
        _ => panic!("Invalid action!"),
    }
}

fn show_help() {
    print!(
        "
pixanim - Tool to help you make pixel art animations

USAGE: pixanim <ACTION>

ACTIONs:
    help     Show this help message.
    make     Build the spritesheet from the image files in the current
             directory.
    play     Build the spritesheet and start playing the resulting
             animation in a new window.
"
    );
}


fn check_paths() {
    let paths = fs::read_dir(".").unwrap();

    let re = Regex::new("^[0-9]+_[0-9]+.png$").unwrap();

    let filtered_paths: Vec<_> = paths
        .filter(|x| re.is_match(x.as_ref().unwrap().file_name().to_str().unwrap()))
        .map(|x| x.unwrap())
        .collect();

    #[cfg(debug_assertions)]
    {
        println!("Printing filtered paths");
        for path in &filtered_paths {
            println!("{}", path.file_name().display());
        }
        println!("------\n");
    }

    // Make it a Vec of owned Strings to make work easier
    let path_strings: Vec<_> = filtered_paths.iter()
        .map(|x| x.file_name().to_str().unwrap().to_string())
        .collect();

    // From now on, we assume that all filtered paths are actual PNG files

    // Check that the row no.1 exists
    let one_counts = path_strings
        .iter()
        .filter(|x| x.as_str().starts_with("1_"))
        .count();

    if one_counts == 0 {
        panic!("Error: row no.1 wasn't found!")
    }

    // Check that no number skips happened:
    // Parse frame numbers into a sorted vector of uints
    // Compare length of vec and largest frame number
    // If they differ, error.

    let mut row_no: u32 = 1;

    let mut ones: Vec<_> = path_strings.iter()
        .filter(|x| x.as_str().starts_with(format!("{row_no}_").as_str()))
        .map(|x| x.split_once('_').unwrap().1)
        .map(|x| x.split_once('.').unwrap().0.parse::<u32>().unwrap())
        .collect();
    ones.sort();
    ones.reverse();

    let largest = *ones.get(0).unwrap();
    let length = ones.len() as u32;
    if length != largest {
        println!("At row no.{row_no}");
        println!("Largest frame index: {largest} != Length: {length}");
        panic!("Number skip happened!")
    }

    #[cfg(debug_assertions)]
    println!("Found {one_counts} instances of row no.{row_no}");

    // Determine number of columns for all rows from first row

    let max_columns: u32 = length;

    // Repeat what was done for row no.1 but for subsequent rows
    loop {
        row_no += 1;

        // Check for existence

        let num_count = path_strings
            .iter()
            .filter(|x| x.as_str().starts_with(format!("{row_no}_").as_str()))
            .count();

        if num_count == 0 {

            // This happens both for when a row was skipped
            // and when it is the last row.
            // We need to check if any other path beyond those checked
            // previously still exists.
            // If at least one does, there's a skip. Error.
            // Else, the row from the last iteration was the last,
            // we break out of the loop.

            let remaining_paths = path_strings
                .iter()
                // Skip the first checked elements from the past rows
                .skip( (row_no as usize - 1) * (max_columns as usize) )
                .count();

            #[cfg(debug_assertions)]
            println!("Number of remaining unchecked paths: {remaining_paths}");

            if remaining_paths == 0  {

                #[cfg(debug_assertions)]
                println!("No unchecked path remaining. Bye!");

                break
            }

            #[cfg(debug_assertions)]
            println!("Paths remaining, not last item. Error!");

            panic!("{}", format!("Error: row no.{row_no} wasn't found!"))
        }

        #[cfg(debug_assertions)]
        println!("Found {num_count} instances of row no.{row_no}");

        // Check that no skips happened

        let mut columns: Vec<_> = path_strings.iter()
            .filter(|x| x.as_str().starts_with(format!("{row_no}_").as_str()))
            .map(|x| x.split_once('_').unwrap().1)
            .map(|x| x.split_once('.').unwrap().0.parse::<u32>().unwrap())
            .collect();
        columns.sort();
        columns.reverse();

        let largest = *columns.get(0).unwrap();
        let length = columns.len() as u32;
        if length != largest {
            println!("At row no.{row_no}");
            println!("Largest frame index: {largest} != Length: {length}");
            panic!("Number skip happened!")
        }

        #[cfg(debug_assertions)]
        println!("No skips happened for row no.{row_no}");

        // Check that column count for the current row matches that of row no.1

        if max_columns != length {
            println!("Number of columns for row no.{row_no} doesn't match first row.");
            panic!("Column count mismatch!");
        }

        #[cfg(debug_assertions)]
        println!("Number of columns of row no.{row_no} matches that of 1st row.");

    }
}

fn make_spritesheet() {
    check_paths();
}

fn visualize_animation() {}
