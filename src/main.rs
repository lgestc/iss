use git2::Repository;
use regex::Regex;
use std::{env, error::Error, fs::File, io::Read, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let repo = Repository::open_from_env()?;
    let head = repo.head()?;

    let branch_name = head.shorthand().ok_or("missing branch name")?;

    let re = Regex::new(r"([A-Z]{2,}\-\d+)").unwrap();

  let captures = re.captures(&branch_name).ok_or("code not found")?;

    let code = captures
        .get(0)
        .ok_or("could not retrieve first matching code")?
        .as_str();

    let default_suffix = String::from("");
    let args: Vec<String> = env::args().collect();
    let wanted_suffix = args.get(1).unwrap_or(&default_suffix);
    let mut suffix = default_suffix.to_string();

    // Create a path to the desired file
    let path_as_string = env::var("HOME").unwrap();
    let path = Path::new(&path_as_string);
    let path = path.join(".config/iss/suffixes");

    let display = path.display();

    if wanted_suffix.trim().chars().count() > 0 && path.exists() {
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();

        if let Err(why) = file.read_to_string(&mut s) {
            panic!("couldn't read {}: {}", display, why);
        }

        let lines = s.split("\n");

        let matching_line: Vec<&str> = lines.filter(|l| l.starts_with(wanted_suffix)).collect();

        let matching_line = matching_line.get(0).unwrap_or(&"");

        let matching_line: Vec<&str> = matching_line.split_whitespace().collect();

        suffix = matching_line.get(1).unwrap_or(&"").to_string();
    }

    println!("{} {} ", code, suffix);

    Ok(())
}
