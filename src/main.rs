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

    let re = Regex::new("^[0-9]_[0-9].png$").unwrap();

    let filtered_paths: Vec<_> = paths
        .filter(|x| re.is_match(x.as_ref().unwrap().file_name().to_str().unwrap()))
        .collect();

    for path in filtered_paths {
        println!("{}", path.unwrap().file_name().display());
    }
}

fn visualize_animation() {}
