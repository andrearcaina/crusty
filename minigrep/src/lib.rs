pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results

    /* could've also done
     *
     * contents
     *     .lines() // get an iterator over the lines of the contents string
     *     .filter(|line| line.contains(query)) // filter the lines that contain the query string
     *     .collect() // collect the filtered lines into a vector
     *
     */
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let result = search(query, contents);
        assert_eq!(vec!["safe, fast, productive."], result);
    }

    #[test]
    fn multiple_results() {
        let query = "Go";
        let contents = "\
Rust:
Rust is great.
Go is also cool.
Go is even cooler.
Go is the coolest.
Rust again.";

        let result = search(query, contents);
        assert_eq!(
            vec![
                "Go is also cool.",
                "Go is even cooler.",
                "Go is the coolest."
            ],
            result
        );
    }

    #[test]
    fn no_results() {
        let query = "Python";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let result = search(query, contents);
        assert!(result.is_empty());
    }

    #[test]
    fn case_insensitive() {
        let query = "go";
        let contents = "\
Rust:
Rust is great.
Go is also cool.
Go is even cooler.
Go is the coolest.
Rust again.";

        let result = search_case_insensitive(query, contents);
        assert_eq!(
            vec![
                "Go is also cool.",
                "Go is even cooler.",
                "Go is the coolest."
            ],
            result
        );
    }
}
