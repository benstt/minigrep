use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the content of the file to a string
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };


    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// The lifetime 'a specifies that the returned vector
// should contain string slices that references the slices
// of the argument *contents*.
//
// This means that the vector returned by
// this function will live as long as the
// contents that are passed into the function.
//
// e.g. Valid
// {
//     let contents = ...; ------------------------------+ 'a
//                                                       |
//     let line = search(query, contents); --------+ 'b  | Valid!
//     println!("line: {}", line);                 |     | The line lives
//                                                 |     | parallel to contents.
// } ----------------goes out of scope-------------+-----+
//
// e.g. Invalid
// {
//     let line; ----------------------------------------+ 'a
//     {                                                 |
//         let contents = ...; --------------------+ 'b  | Invalid!
//         line = search(query, contents);         |     | The line depends on the
//         println!("line: {}", line);             |     | contents, but the refe-
//                                                 |     | rence is now invalid.
//     } ------------goes out of scope-------------+     | 
//                                                       |
//     println!("line: {}", line); BIG PROBLEM!          |
// } ----------------goes out of scope-------------------+
//
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


// Group data together
#[derive(Debug)]
pub struct Config {
    pub query: String, // The string to search for
    pub filename: String, // The filename to search the string in
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // Make sure the user provides 2 arguments
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Make a clone of the strings from the argument.
        // In that way, we don't transfer ownership and the
        // values in args are still available.
        // However, using clone() is a bit unefficient,
        // but that doesn't matter for now.
        let query = args[1].clone(); // The index 0 is used for the path
        let filename = args[2].clone();

        // Get the environment variable to set the case_sensitive.
        //
        // CASE_INSENSITIVE will tell us if the user DOES NOT want to
        // search for a query regardless of capital letters.
        // is_err() will return if the environment variable
        // is indeed set or not set. If var() returns Err, then it
        // means that the CASE_INSENSITIVE envar is not set.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."],
                   search_case_insensitive(query, contents)
        );
    }
}
