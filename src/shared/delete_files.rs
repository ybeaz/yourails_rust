use std::fs;

pub fn delete_files(paths: &Vec<String>) -> Result<(), Vec<(String, std::io::Error)>> {
    let mut errors = Vec::new();

    for path in paths {
        if let Err(e) = fs::remove_file(path) {
            errors.push((path.clone(), e));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
