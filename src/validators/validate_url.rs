use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use regex::Regex;


/**
    Checks validity of url in parameter
    @param url_to_test: the url to validate
    @param path_to_whitelist: path to top level domain whitelist file
    it should have 1 top level domain per line

    If the given whitelist is empty, the default mode is accepting every top level domain
    If the given whitelist don't exist, the program works with default mode
 */
pub fn validate_url(url_to_test: &str, path_to_whitelist: Option<String>) -> bool {
    let regex_protocol = r"([a-z0-9]{3,}://)?";
    let regex_subdomain = r"([a-z0-9\-.]{1,}\.)+";
    let regex_domain = r"([a-z0-9_\-]{1,})";
    let regex_top_level_domain = r"(\.([a-z]{2,}))";
    let mut regex_url = format!("{}{}{}", regex_protocol, regex_subdomain, regex_domain);
    let mut unwrapped_white_list: Vec<String>= Vec::new();
    match path_to_whitelist {
        Some(path) => {
            match lines_from_file(Path::new(&path)) {
                Ok(vector) => {unwrapped_white_list = vector}
                Err(_) => {}
            }
        },
        _ => {}
    }
    if unwrapped_white_list.len() > 0 {
        regex_url = format!("{}\\.(", regex_url);
        for i in 0..unwrapped_white_list.len() {
            regex_url = format!("{}({})", regex_url, unwrapped_white_list.get(i).unwrap());
            if i != unwrapped_white_list.len()-1 {
                regex_url = format!("{}|", regex_url);
            }
        }
        regex_url = format!("{})", regex_url);
        println!("{}",regex_url);
    }
    else {
        regex_url = format!("{}{}", regex_url, regex_top_level_domain);
    }
    let regex = Regex::new(&*regex_url).unwrap();
    regex.is_match(url_to_test)


}

//source: https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings
fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>, Error> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    Ok(buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect())

}

// TODO : implement unit testing
#[cfg(test)]
mod tests {
    use crate::validate_url;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn empty_url_fail(){
        assert_eq!(false , validate_url("", None));
    }

    #[test]
    fn basic_url_success(){
        assert_eq!(true , validate_url("www.google.ch", None));
    }

    #[test]
    fn no_subdomain_fail(){
        assert_eq!(false , validate_url("google.ch", None));
    }

    #[test]
    fn no_subdomain_with_protocol_fails(){
        assert_eq!(false , validate_url("http://google.ch", None));
    }

    #[test]
    fn top_level_domain_without_whitelist_works(){
        assert_eq!(true, validate_url("never.gonnagiveyou.up", None));
    }


    #[test]
    fn no_top_level_domain_fails(){
        assert_eq!(false , validate_url("www.google", None))
    }

    #[test]
    fn no_top_level_no_subdomain_fails(){
        assert_eq!(false , validate_url("google", None))
    }

    #[test]
    fn basic_url_with_whitelist(){
        let path_to_whitelist :String = "src/resources/authorised_domain".to_owned();
        assert_eq!(true , validate_url("www.google.ch", Some(path_to_whitelist)))
    }




}
