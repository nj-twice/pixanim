use regex::Regex;
use std::env;
use std::fs;
use image::{ImageReader, RgbaImage, imageops::overlay};

const FIRST_FRAME: &str = "1_1.png";
const MARGIN: u32 = 2;

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
        // TODO: add an edit subcommand: deconstructs a sprite sheet into properly named files, wait for edits, then rebuild the sprite sheet
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


fn check_paths(image_paths: &Vec<String>) {
    // Here, we don't care that the files are actually PNGs or not.
    // We just enforce the naming convetion.

    // Check that the row no.1 exists
    let one_counts = image_paths
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

    let mut ones: Vec<_> = image_paths.iter()
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

        let num_count = image_paths
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

            let remaining_paths = image_paths
                .iter()
                // Skip the first checked elements from the past rows
                .skip( (row_no as usize - 1) * (max_columns as usize) )
                .count();

            #[cfg(debug_assertions)]
            println!("Number of remaining unchecked paths: {remaining_paths}");

            if remaining_paths == 0  {

                #[cfg(debug_assertions)]
                println!("No unchecked path remaining. Continuing.");

                break
            }

            #[cfg(debug_assertions)]
            println!("Paths remaining, not last item. Error!");

            panic!("{}", format!("Error: row no.{row_no} wasn't found!"))
        }

        #[cfg(debug_assertions)]
        println!("Found {num_count} instances of row no.{row_no}");

        // Check that no skips happened

        let mut columns: Vec<_> = image_paths.iter()
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

struct Point {
    x: i64,
    y: i64
}

fn make_spritesheet() {
    let paths = get_images_paths();
    check_paths(&paths);
    check_image_dimensions(&paths);

    // Actually build the spritesheet

    let (mut width, mut height) = ImageReader::open(FIRST_FRAME)
        .unwrap().into_dimensions().unwrap();

    let columns: u32 = paths.iter()
        .filter(|x| x.starts_with("1_"))
        .count()
        .try_into().unwrap();
    let total_files: u32 = paths.iter().count().try_into().unwrap();
    let rows: u32 = total_files / columns;

    // Add a margin to avoid flickering borders in animation
    // Double to account for centering.
    width += MARGIN * 2;
    height += MARGIN * 2;

    let mut insert_point = Point {x: 0 as i64, y: 0 as i64};
    let mut final_image = RgbaImage::new(width * columns, height * rows);

    for frame in paths {

        // We don't need to sort the path list.
        // We can determine the insert point coordinates based on
        // frame numbers.
        // row 1, col 1 => x = 0, y = 0
        // row 1, col 2 => x = frame width, y = 0
        // row 2, col 1 => x = 0, y = frame height
        // row 2, col 2 => x = frame width, y = frame height
        // row 3, col 1 => x = 0, y = frame height * 2
        // ...

        let row = frame.as_str().split_once('_')
                .unwrap().0.parse::<i64>().unwrap();
        let col = frame.as_str().split_once('_')
                .unwrap().1
                .split_once('.').unwrap().0.parse::<i64>().unwrap();

        insert_point.x = (col - 1) * (width as i64) + (MARGIN as i64);
        insert_point.y = (row - 1) * (height as i64) + (MARGIN as i64);

        let overlay_image = ImageReader::open(frame).unwrap().decode().unwrap();
        overlay(&mut final_image, &overlay_image, insert_point.x, insert_point.y);
    }

    final_image.save("spritesheet.png");

    println!("Frame dimensions: {width} × {height}")
}

fn check_image_dimensions(paths: &Vec<String>) {
    let init_dimensions = ImageReader::open(FIRST_FRAME)
        .unwrap().into_dimensions();

    match init_dimensions {
        Err(_) => {
             println!("Error: probably not an image");
             panic!("Image error!");
             }
        Ok(_) => ()
    }

    let init_dimensions = init_dimensions.unwrap();

    #[cfg(debug_assertions)]
    {
        println!("Printing found dimensions");
        dbg!(init_dimensions);
    }

    for file in paths {
        let dimensions = ImageReader::open(file).unwrap().into_dimensions();

        match dimensions {
            Err(_) => {
             println!("Error: {file} is probably not an image");
             panic!("Image error!");
            }
            Ok(_) => ()
        }

        let dimensions = dimensions.unwrap();

        if dimensions != init_dimensions {
            let x = dimensions.0;
            let y = dimensions.1;
            panic!("Dimensions of {file} ({x}×{y}) don't match those of first frame.")
        }
    }
}

fn get_images_paths() -> Vec<String> {
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

    path_strings
}

// TODO: use macroquad to play animation
//       - interactively changeg FPS
//       - select which row (animation)
//       - select columns range (frames)
//       - display some info from a generated info file?
// TODO: detect existence of an already generated sprite sheet
fn visualize_animation() {}
