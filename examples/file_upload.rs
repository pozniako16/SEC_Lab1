use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Mutex;
use infer::MatcherType;
use uuid::Uuid;

use read_input::prelude::*;
use Lab01::{validate_file, validate_uuid};
//use lab01_2022_input_validation::*;

// Use the hashmap as follows:
// ```
// let map = HASHMAP.lock().unwrap();
// ```

static END_POINT_URL: &str = "sec.upload";
static VIDEO_FOLDER: &str = "videos";
static IMAGE_FOLDER: &str = "images";

struct FileInfos {
    uuid: Uuid,
    file_type: MatcherType,
    file_upload_url: String
}

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<Uuid, FileInfos>> = Mutex::new(HashMap::new());
}

fn file_upload_handler() {
    loop {
        match input::<String>().repeat_msg("Please enter the path to an image or video file (empty returns to previous menu):").get() {
            file_path => {
                if file_path.len() == 0 {
                    break
                }
                match validate_file(&file_path, None, None) {
                    Ok(file_valid) => {
                        if file_valid {
                            let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, file_path.as_bytes());
                            let mut map = HASHMAP.lock().unwrap();
                            if map.contains_key(&uuid){
                                println!("File already exists, can't overwrite !");
                                break
                            } else {
                                //retrieve file type
                                let file_type = infer::get_from_path(&file_path).unwrap().unwrap().matcher_type();
                                let mut file_upload_url = "".to_string();
                                match file_type {
                                    MatcherType::Image => { file_upload_url = format!("{}/{}/{}", END_POINT_URL, IMAGE_FOLDER, file_path)}
                                    MatcherType::Video => { file_upload_url = format!("{}/{}/{}", END_POINT_URL, VIDEO_FOLDER, file_path)}
                                    _ => {}

                                }
                                let file_infos = FileInfos {
                                    uuid,
                                    file_type,
                                    file_upload_url
                                };
                                map.insert(uuid, file_infos);
                                println!("File uploaded successfully, UUID : {:?}", uuid);
                                break
                            }
                        } else {
                            println!("Invalid file content !");
                        }
                    }
                    Err(_) => {
                        println!("Invalid file contents !");
                    }
                }
            }
        }
    }
}

// TODO: IMPLEMENT VERIFY LOGIC
fn file_verify_handler() {
    loop {
        match input::<String>().repeat_msg("Please enter the UUID to check (empty returns to previous menu):").get() {
            uuid_str => {
                if uuid_str.len() == 0 {
                    break
                }
                match validate_uuid(&uuid_str) {
                    true => {
                        let map = HASHMAP.lock().unwrap();
                        let uuid = Uuid::from_str(&uuid_str).unwrap();

                        match map.contains_key(&uuid) {
                            false => {
                                println!("File does not exist");
                            }
                            true => {
                                let file = map.get(&uuid).unwrap();
                                let mut type_str = "a video";
                                if file.file_type == MatcherType::Image {
                                    type_str = "an image"
                                }
                                println!("File {:?} exists, it is {} file", file.uuid, type_str);
                                break
                            }
                        }
                    }
                    false => {
                        println!("UUID is invalid !")
                    }
                }
            }
        }
    }
}


fn get_url_handler() {
    loop {
        match input::<String>().repeat_msg("Please enter the UUID to get (empty returns to previous menu):").get() {
            uuid_str => {
                if uuid_str.len() == 0 {
                    break
                }
                if validate_uuid(&uuid_str) {
                    let uuid = Uuid::from_str(&uuid_str).unwrap();
                    let map = HASHMAP.lock().unwrap();
                    if map.contains_key(&uuid){
                        let file_infos = map.get(&uuid).unwrap();
                        println!("{}", file_infos.file_upload_url);
                        break
                    }
                    println!("Can't find the file associated with uuid");
                } else {
                    println!("UUID does not have the right format!");
                }
            }
        }
    }
}

fn main() {
    println!("Welcome to the super secure file upload tool !");
    loop {
        match input::<i32>().repeat_msg("Please select one of the following options to continue :\n1 - Upload a file\n2 - Verify file exists\n3 - Get file URL\n0 - Exit\nYour input ? [0-3]")
            .min_max(0, 3).get() {
            0 => {
                println!("Goodbye!");
                break
            },
            1 => file_upload_handler(),
            2 => file_verify_handler(),
            3 => get_url_handler(),
            _ => panic!("Invalid input"),
        }
    }
}
