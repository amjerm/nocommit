use clap::Parser;
use grep::{
    cli,
    printer::{ColorSpecs, StandardBuilder},
    regex::RegexMatcherBuilder,
    searcher::SearcherBuilder,
};
use std::{
    ffi::OsString,
    io::{BufRead, BufReader},
    process::{exit, Command, Stdio},
};
use termcolor::{self, ColorChoice};

pub struct Settings {
    pattern: String,
    staged_only: bool,
}

#[derive(Parser)]
#[command()]
pub struct Cli {
    /// Search unstaged files as well
    #[arg(short, long)]
    all_files: bool,
}

impl Settings {
    pub fn new(pattern: String, cli: Cli) -> Self {
        Self {
            pattern,
            staged_only: !cli.all_files,
        }
    }
}

pub fn run(settings: Settings) {
    let file_list = get_staged_file_list(settings.staged_only);
    let mut found_matches: u16 = 0;
    println!("nocommit searching changed files...");

    println!("\nFiles to search:");
    for file in &file_list {
        println!("{}", file);
    }
    println!("\n");

    let pattern: &str = &settings.pattern;
    for file in file_list {
        match search_file_for_pattern(pattern, OsString::from(file)) {
            true => found_matches += 1,
            _ => (),
        }
    }

    if found_matches > 0 {
        if found_matches == 1 {
            println!("\n\x1b[41mFound matches in {} file!\x1b[0m", found_matches);
        } else {
            println!("\n\x1b[41mFound matches in {} files!\x1b[0m", found_matches);
        }
        exit(1)
    } else {
        println!("\nNo matches found.")
    }
}

fn search_file_for_pattern(pattern: &str, file_path: OsString) -> bool {
    let matcher = RegexMatcherBuilder::new()
        .case_insensitive(true)
        .line_terminator(Some(b'\n'))
        .build(pattern)
        .expect("Could not build matcher");
    let mut searcher = SearcherBuilder::new().build();
    let mut printer = StandardBuilder::new()
        .color_specs(ColorSpecs::default_with_color())
        .build(cli::stdout(ColorChoice::Auto));

    let _ = searcher.search_path(
        &matcher,
        &file_path,
        printer.sink_with_path(&matcher, &file_path),
    );

    printer.has_written()
}

fn get_staged_file_list(staged_only: bool) -> Vec<String> {
    let mut diff_args = vec!["diff-index", "--name-status"];

    if staged_only {
        diff_args.push("--cached");
    }

    diff_args.push("HEAD");

    let mut diff_result = Command::new("git")
        .args(diff_args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Error getting diff");

    let cut_result = Command::new("cut")
        .arg("-c")
        .arg("3-")
        .stdin(diff_result.stdout.take().unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Error cutting diff");

    let _ = diff_result.wait().expect("Failed to wait for diff");
    let output2 = BufReader::new(cut_result.stdout.expect("Failed to get command 2 stdout"));
    let staged_files: Vec<String> = output2.lines().map(|line| line.unwrap()).collect();

    staged_files
}
