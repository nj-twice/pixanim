use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.iter().count() - 1 {
        0 => panic!("No arguments supplied!"),
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

fn make_spritesheet() {
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

    let mut row_no = 1;

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

    // Determine number of columns for all rows from first row

    // loop {
    //     row_no += 1;
    // }
}

fn visualize_animation() {}
