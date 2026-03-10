// 1. Iterate through each line of the contents.
// 2. Check whether the line contains our query string.
// 3. If it does, add it to the list of values we’re returning.
// 4. If it doesn’t, do nothing.
// 5. Return the list of results that match.
// NOTE: 'a = the data returned by the search function will live as long as the
// data passed into the search function in the contents argument
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in contents.lines() {
                if line.contains(query) {
                        results.push(line);
                }
        }

        results
}

pub fn search_case_insensitive<'a>(
        query: &str,
        contents: &'a str,
) -> Vec<&'a str> {
        let query = query.to_lowercase();
        let mut results = Vec::new();

        for line in contents.lines() {
                // to_lowercase will handle basic Unicode, it won’t be 100
                // percent accurate
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
        fn case_sensitive() {
                let query = "duct";
                let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

                assert_eq!(
                        vec!["safe, fast, productive."],
                        search(query, contents)
                );
        }

        #[test]
        fn case_insensitive() {
                let query = "rUsT";
                let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

                assert_eq!(
                        vec!["Rust:", "Trust me."],
                        search_case_insensitive(query, contents)
                );
        }
}
