use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

pub fn save_file_to_disk(path: PathBuf, text: String) {
    let save_result = write(path, text);

    match save_result {
        Ok(_) => println!("Save file successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }
}

pub fn load_file_from_disk(path: PathBuf) -> String {
    let read_result = read_to_string(path);

    match read_result {
        Ok(contents) => contents,
        Err(_) => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, remove_file};

    #[test]
    fn test_save_file_to_disk() {
        let test_path = PathBuf::from("test_file.txt");
        let test_content = String::from("Hello, Chat!");

        save_file_to_disk(test_path.clone(), test_content.clone());

        let saved_content = fs::read_to_string(&test_path).unwrap();

        assert_eq!(saved_content, test_content);

        let _ = remove_file(test_path);
    }

    #[test]
    fn test_load_file_from_disk() {
        let test_path = PathBuf::from("test_file.txt");
        let test_content = String::from("Hello, Chat!");
        let _ = fs::write(test_path.clone(), test_content.clone());

        let saved_content = load_file_from_disk(test_path.clone());

        assert_eq!(saved_content, test_content);

        let _ = remove_file(test_path);
    }
}
