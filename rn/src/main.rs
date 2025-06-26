use clap::{Arg, Command};
use std::fs;
use std::path::Path;

fn main() {
    let matches = Command::new("rename-util")
        .about("Batch or single file renamer")
        .arg(Arg::new("files")
            .help("Input file(s) and optionally output file if single rename")
            .required(true)
            .num_args(1..))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .help("Output file name or base name for batch")
            .required(false)
            .value_name("OUTPUT"))
        .get_matches();

    let files: Vec<&String> = matches.get_many::<String>("files").unwrap().collect();
    let output_opt = matches.get_one::<String>("output");

    match files.len() {
        1 => {
            eprintln!("Error: Output file must be specified for single input.");
            std::process::exit(1);
        }
        2 => {
            // If -o is given, override second argument for output
            if let Some(output) = output_opt {
                rename_single(files[0], output);
            } else {
                // Treat second argument as output file name
                rename_single(files[0], files[1]);
            }
        }
        _ => {
            // More than 2 inputs => multiple rename, -o required
            if output_opt.is_none() {
                eprintln!("Error: For multiple input files (>2), -o <output_base> is required.");
                std::process::exit(1);
            }
            rename_multiple(&files, output_opt.unwrap());
        }
    }
}

fn rename_single(input: &str, output: &str) {
    if !Path::new(input).exists() {
        eprintln!("Error: File '{}' does not exist.", input);
        std::process::exit(1);
    }
    if let Err(e) = fs::rename(input, output) {
        eprintln!("Error renaming '{}': {}", input, e);
        std::process::exit(1);
    }
}

fn rename_multiple(inputs: &[&String], base: &str) {
    for (i, input) in inputs.iter().enumerate() {
        if !Path::new(input).exists() {
            eprintln!("Warning: File '{}' does not exist. Skipping.", input);
            continue;
        }
        let new_name = if i == 0 {
            base.to_string()
        } else {
            format!("{}{}", base, i + 1)
        };
        if let Err(e) = fs::rename(input, &new_name) {
            eprintln!("Error renaming '{}': {}", input, e);
        }
    }
}

