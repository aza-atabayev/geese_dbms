use std::fs;

pub fn get_file_names(path: &str) -> Vec<String> {
    let mut file_names = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_name_os_str) = entry.file_name().into_string() {
                    file_names.push(file_name_os_str);
                }
            }
        }
    }
    file_names
}
