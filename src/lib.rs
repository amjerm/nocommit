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

pub fn run(pattern: &str) {
    let file_list = get_staged_file_list();
    println!("file_list is: {:?}", file_list);
    let mut found_matches = false;
    for file in file_list {
        println!("searching {}...", file);
        match search_file_for_pattern(pattern, OsString::from(file)) {
            true => found_matches = true,
            _ => (),
        }
    }
    if found_matches {
        exit(1)
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

fn get_staged_file_list() -> Vec<String> {
    let mut diff_result = Command::new("git")
        .arg("diff-index")
        .arg("--name-status")
        .arg("--cached")
        .arg("HEAD")
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
