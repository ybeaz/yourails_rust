use std::fs;
use std::path::Path;

pub fn read_directory_files(
    input: &str,
    extensions: &[&str],
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let directory = Path::new(input);

    if !directory.is_dir() {
        return Err(format!("Not a directory: {input}").into());
    }

    let mut files = Vec::new();

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        // Only files, not directories.
        if !path.is_file() {
            continue;
        }

        // Get extension.
        let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
            continue;
        };

        // Case-insensitive extension matching.
        if extensions
            .iter()
            .any(|ext| ext.eq_ignore_ascii_case(extension))
        {
            files.push(path.to_string_lossy().into_owned());
        }
    }

    Ok(files)
}
