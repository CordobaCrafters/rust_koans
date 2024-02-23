use std::fs;
use std::io;
use std::process::Command;

use clap::Parser;
use colored::Colorize;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn get_koans() -> io::Result<Vec<String>> {
    let content = fs::read_to_string("koans.txt")?;

    let koans: Vec<String> = content
        .lines()
        .map(|line| line.to_string())
        .filter(|line| !line.is_empty())
        .collect();

    Ok(koans)
}

fn get_tests_for_koan(koan: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let test_file_path = format!("src/{}.rs", koan);
    let content = fs::read_to_string(test_file_path)?;

    let re = Regex::new(r"fn (test_\w+)\(")?;
    let tests = re
        .captures_iter(&content)
        .filter_map(|cap| cap.get(1))
        .map(|m| format!("{}::test::{}", koan, m.as_str()))
        .collect::<Vec<String>>();

    Ok(tests)
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let all_tests = get_koans()
        .unwrap_or_else(|_| vec![])
        .into_iter()
        .flat_map(|koan| get_tests_for_koan(koan).unwrap_or_else(|_| vec![]))
        .collect::<Vec<String>>();

    let all_tests_count = all_tests.len();

    for (tests_passed, test_name) in all_tests.iter().enumerate() {
        let output = Command::new("cargo")
            .args(["test", &test_name.as_str()])
            .output()?;

        print!("Running test: {} ... ", test_name);
        if !output.status.success() {
            println!("{}", "failed.".red());
            if args.verbose {
                eprintln!("{}", String::from_utf8_lossy(&output.stdout));
            }

            println!("Passed {} tests of {}.", tests_passed, all_tests_count);
            std::process::exit(1);
        }

        println!("{}", "success.".green());
    }

    println!("Passed {} tests of {}.", all_tests_count, all_tests_count);
    println!("All tests passed!");
    Ok(())
}
