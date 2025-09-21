use std::env;

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

fn make_spritesheet() {}

fn visualize_animation() {}
