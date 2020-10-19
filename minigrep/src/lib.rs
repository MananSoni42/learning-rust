use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub fname: String,
    pub case_insensitive: bool,
    pub exact: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() != 3 { return Err(format!("expected: 2 got: {}", args.len()-1)); }
        let case = env::var("CASE_INSENSITIVE").is_ok();
        let exact = env::var("EXACT").is_ok();
        Ok(Config {  fname: args[1].clone(),
                    query: args[2].clone(),
                    case_insensitive: case,
                    exact: exact })
    }
}

pub fn run(cfg: &Config) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(&cfg.fname)?;

    println!("Matched: {:#?}", search(&cfg, &data));
    Ok(())
}

pub fn search<'a>(cfg: &Config, content:&'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = cfg.query.to_string();

    for line in content.lines() {

        let mut cond: bool = line.contains(&query);

        if cfg.case_insensitive {
            cond = line.to_lowercase().contains(&query.to_lowercase());
        }

        if cfg.exact {
            let pre_query = query.clone();
            cond = line.contains(&(" ".to_owned() + &query + " ")) ||
                    line.starts_with(&(pre_query + " ")) || line.ends_with(&(" ".to_owned() + &query));
        }

        if cond {
            results.push(line.trim());
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_res() {
        let contents = "\
        Rust
        safe, fast, productive!
        Pick three!";

        let cfg = Config {
            query: "duct".to_string(),
            fname: "test".to_string(),
            case_insensitive: false,
            exact: false,
        };

        assert_eq!(vec!["safe, fast, productive!"],
                        search(&cfg, contents));
    }

    #[test]
    fn case_sensitive() {
        let content = "\
        even Duct tape can't make this test case pass
        written using Rust
        ";

        let cfg = Config {
            query: "duct".to_string(),
            fname: "test".to_string(),
            case_insensitive: false,
            exact: false,
        };

        assert_eq!(Vec::<String>::new(),
                        search(&cfg, content)); // vec![] does not work
                                                                // bcoz of insufficient type info
    }

    #[test]
    fn case_insensitive() {
        let content = "\
        even Duct tape can't make this test case pass
        written using Rust
        just trust me.
        ";

        let cfg = Config {
            query: "RuSt".to_string(),
            fname: "test".to_string(),
            case_insensitive: true,
            exact: false,
        };

        assert_eq!(vec!["written using Rust",
                        "just trust me."],
                        search(&cfg, content));
    }

    #[test]
    fn case_not_exact() {
        let content = "\
        they are going to catch this line
        this line with 'the' is also caught
        the line will also be caught tho
        this one won't be
        ";

        let cfg = Config {
            query: "the".to_string(),
            fname: "test".to_string(),
            case_insensitive: false,
            exact: false,
        };

        assert_eq!(vec!["they are going to catch this line",
                        "this line with 'the' is also caught",
                        "the line will also be caught tho"],
                        search(&cfg, content));
    }

    #[test]
    fn case_exact() {
        let content = "\
        they are not going to catch this line
        this line with 'the' is also NOT caught
        middle the should be caught
        but then this one shouldnt be caught?
        the line will also be caught tho
        this one won't be
        even one at end will be caught the
        ";

        let cfg = Config {
            query: "the".to_string(),
            fname: "test".to_string(),
            case_insensitive: false,
            exact: true,
        };

        assert_eq!(vec!["middle the should be caught",
                        "the line will also be caught tho",
                        "even one at end will be caught the"],
                        search(&cfg, content));
    }
}
