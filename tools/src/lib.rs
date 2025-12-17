use std::{env, fs, path};


pub fn read_input_file() -> String {
    let path = format!(
        "{}{}src{}input.txt",
        env::current_dir()
            .expect("Failed to get current directory")
            .to_str()
            .expect("Failed to convert path to string"),
        path::MAIN_SEPARATOR,
        path::MAIN_SEPARATOR
    );

    let contents_with_bom =
        fs::read_to_string(&path).expect(&format!("Failed to read file: {}", path));
    contents_with_bom.trim_start_matches('\u{FEFF}').to_string()
}

pub fn write_result_file(result: &str){
    let path = format!(
        "{}{}src{}result.txt",
        env::current_dir()
            .expect("Failed to get current directory")
            .to_str()
            .expect("Failed to convert path to string"),
        path::MAIN_SEPARATOR,
        path::MAIN_SEPARATOR
    );
    
    fs::write(&path, result).expect(&format!("Failed to write file: {}", path));
}