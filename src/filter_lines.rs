use regex::Regex;


pub fn create_line_regex(import_matcher: &str) -> Regex {
    let re = Regex::new(import_matcher).unwrap();
    return re;
}

