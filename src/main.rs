use forgescript::count_files_and_lines;
use forgescript::save_results_to_json;
use clap::{App, Arg}; // Add missing import statement

fn main() {
    let matches = App::new("ForgeScript")
        .version("0.1.0")
        .author("T-Bit Bots")
        .about("Analyzes and improves JavaScript projects")
        .arg(Arg::with_name("input")
            .short('i')
            .long("input")
            .value_name("DIRECTORY")
            .help("Sets the input directory to use")
            .takes_value(true))
        .get_matches();

    // Handle the input directory argument
    if let Some(i) = matches.value_of("input") {
        println!("Analyzing files in directory: {}", i);
        // Call the imported function
        let file_count = count_files_and_lines(i);
        save_results_to_json(file_count).expect("Failed to save results to file");
    }
}
