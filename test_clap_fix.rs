use clap::{Command, Arg, ArgMatches};

fn main() {
    let matches = Command::new("test-app")
        .version("1.0.0")
        .about("Test clap fixes")
        .arg(
            Arg::new("input")
                .help("Input files")
                .num_args(1..)
                .required(true)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Verbose mode")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // Test the new API usage
    let input_files: Vec<String> = matches.get_many::<String>("input")
        .unwrap()
        .map(|s| s.clone())
        .collect();
    
    let output_file = matches.get_one::<String>("output");
    let verbose = matches.get_flag("verbose");

    println!("Input files: {:?}", input_files);
    println!("Output file: {:?}", output_file);
    println!("Verbose: {}", verbose);
}
