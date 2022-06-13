use std::fs;
use std::env;
extern crate colored;
extern crate fstream;
extern crate walkdir;
use colored::*;
use std::path::Path;
use walkdir::WalkDir;
const DIRS:[&str; 5] = [
    "C:\\Users\\mazav\\OneDrive\\Documents\\PDF",
    "C:\\ZIP",
    "C:\\installation",
    "C:\\Users\\mazav\\OneDrive\\Images\\SVG",
    "C:\\Users\\mazav\\OneDrive\\Images", 
];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments");
    } else if args.len() == 3 {
        let q = &args[1];
        if q == "-f" {
            search_file(args);
        } else if q == "-q" {
            search_string(args);
        } else {
            println!("Unknown argument");
        }

    } else if args.len() > 3 {
        println!("Too many arguments");
    }
}

fn search_string(args: Vec<String>) {
    let path = &".".to_string();
    let string = &args[2];
    println!("Searching for {} in {}", string.green().bold(), path.italic());
    check_dir(path, string);
}

fn search_file(args: Vec<String>) {
    for dir in DIRS {
        let paths = fs::read_dir(dir).unwrap();
        for path in paths {
            let patha = path.unwrap().path();
            let file_name = patha.file_name().unwrap().to_str().unwrap();
            if file_name.contains(&args[2]) || file_name.contains(&args[2].to_lowercase()) || file_name.contains(&args[2].to_uppercase()) || file_name.contains(&title(&args[2])) {
                println!("{:?}", patha.display());
            }
        }
    }
}

fn title(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
    }
}

fn check_dir(path: &str, query: &str) {
    let mut total_files_scanned = 0;
    for (fl_no, file) in WalkDir::new(path)
        .into_iter()
        .filter_map(|file| file.ok())
        .enumerate()
    {
        if file.metadata().unwrap().is_file() {
            match fstream::contains(file.path(), query) {
                Some(b) => {
                    if b {
                        check_file(file.path(), query);
                    }
                }
                None => println!("Error in walking Dir"),
            }
        }
        total_files_scanned = fl_no;
    }
    println!(
        "Total scanned files {}",
        total_files_scanned.to_string().bold()
    );

}

fn check_file(file_path: &Path, query: &str) {
    println!(
        "In file {}\n",
        file_path.display().to_string().magenta().italic()
    );
    
    match fstream::read_lines(file_path) {
        Some(lines) => {
            for (pos, line) in &mut lines.iter().enumerate() {
                if line.contains(query) {
                    let line: String = line.trim().chars().take(2000).collect();

                    print!("{}", "Line ".green().bold());
                    print!("{0: <6} ", pos.to_string().cyan());
                    println!("=> {}", line.blue());
                     
                }
            }
        }
        None => println!("Error in reading File"),
    }
    println!("");
}