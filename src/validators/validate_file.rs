use std::io::Error;
use infer::MatcherType;



/**
Validate if files has given type/Extension
@param path_to_file: path to the file we want to validate
@param file_type: String, Optional parameter, given, we check if the file has this extension
@param file_group: MatcherType, Optionnal parameter, given, we check if the file belongs to this group

If no optional argument is given, the default mode is to check if the file is whether an image
or a video, returns true if it's the case, false otherwise

The priority in which the optional arguments are treated is:
    1- file extension
    2- file group
If both are filled, file group will be ignored

REMARK ON TESTING: you need files to be checked for the tests to work, here is how it goes:
    - first download the src/resources folder from the following repository
    - copy paste it to the src folder
    - launch the tests
 **/
pub fn validate_file(path_to_file: &str, file_extension: Option<&str>, file_group: Option<MatcherType>) -> Result<bool, Error> {
    let kind = infer::get_from_path(path_to_file)?;
    match kind {
        None => {Ok(false)}
        Some(unwrapped_kind) => {
            match file_extension {
                None => {}
                Some(file_extension) => {
                    if file_extension.len() != 0 {
                        if unwrapped_kind.mime_type() == file_extension {
                            return Ok(false);
                        }
                        return Ok(true)
                    }
                }
            }
            return match file_group {
                None => {
                    if unwrapped_kind.matcher_type() != MatcherType::Image &&
                        unwrapped_kind.matcher_type() != MatcherType::Video {
                        Ok(false)
                    } else {
                        Ok(true)
                    }
                }
                Some(group) => {
                    if unwrapped_kind.matcher_type() == group {
                        return Ok(true)
                    }
                    Ok(false)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use infer::{Matcher, MatcherType};
    use crate::validate_file;

    static IMAGE_FOLDER: &str = "src/resources/images/";
    static VIDEO_FOLDER: &str = "src/resources/videos/";
    static OTHER_FOLDER: &str = "src/resources/other/";

    #[test]
    fn basic_image_works(){
        let path = format!("{}image_test.jpg", IMAGE_FOLDER);
        match validate_file(&path,None, None) {
            Ok(res) => {assert!(res)}
            Err(_) => {assert!(false)}
        }
    }

    #[test]
    fn basic_video_works(){
        let path = format!("{}video_test.avi", VIDEO_FOLDER);
        match validate_file(&path,None, None) {
            Ok(res) => {assert!(res)}
            Err(_) => {assert!(false)}
        }
    }

    #[test]
    fn non_existing_file_returns_error(){
        match validate_file("don't exist", None, None){
            Ok(_) => {assert!(false)}
            Err(_) => {assert!(true)}
        }
    }

    #[test]
    fn specify_group_bypasses_default_mode(){
        let path = format!("{}test_audio.mp3", OTHER_FOLDER);
        match validate_file(&path, None, Some(MatcherType::Audio)){
            Ok(res) => {assert!(res)}
            Err(_) => {assert!(false)}
        }
    }

    #[test]
    fn specify_extension_with_default_group_works(){
        let path = format!("{}image_test.jpg", IMAGE_FOLDER);
        match validate_file(&path,Some("jpg"), None) {
            Ok(res) => {assert!(res)}
            Err(_) => {assert!(false)}
        }
    }

    #[test]
    fn specify_extension_not_image_video_bypasses_default_mode(){
        let path = format!("{}test_audio.mp3", OTHER_FOLDER);
        match validate_file(&path, Some("mp3"), None){
            Ok(res) => {assert!(res)}
            Err(_) => {assert!(false)}
        }
    }

    #[test]
    fn specify_group_and_extension_ignores_group(){
        let path = format!("{}test_audio.mp3", OTHER_FOLDER);
        match validate_file(&path, Some("mp3"), Some(MatcherType::Video)){
            Ok(res) => {assert!(res)}
            Err(_) => {assert!(false)}
        }
    }

    #[test]
    fn empty_extension_return_ignores_it(){
        let path = format!("{}image_test.jpg", IMAGE_FOLDER);
        match validate_file(&path,Some(""), None) {
            Ok(res) => {assert!(res)}
            Err(_) => {assert!(false)}
        }
    }
}
