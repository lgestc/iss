use git2::Repository;
use regex::Regex;
use std::error::Error;

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

    println!("{}", code);

    Ok(())
}
