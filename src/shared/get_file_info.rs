use std::path::Path;
use std::fs;
use image::GenericImageView;

#[derive(Debug)]
pub struct FileInfo {
    pub path: String,
    pub size_bytes: u64,
    pub is_image: bool,
    pub image_width: Option<u32>,
    pub image_height: Option<u32>,
    pub image_format: Option<String>,
    pub color_type: Option<String>,
}

pub fn get_file_info<P: AsRef<Path>>(path: P) -> Result<FileInfo, Box<dyn std::error::Error>> {
    let path = path.as_ref();

    let metadata = fs::metadata(path)?;

    let size_bytes = metadata.len();

    match image::open(path) {
        Ok(img) => {
            let (width, height) = img.dimensions();

            Ok(FileInfo {
                path: path.to_string_lossy().to_string(),
                size_bytes,
                is_image: true,
                image_width: Some(width),
                image_height: Some(height),
                image_format: image::ImageFormat::from_path(path)
                    .ok()
                    .map(|format| format!("{:?}", format)),
                color_type: Some(format!("{:?}", img.color())),
            })
        }

        Err(_) => Ok(FileInfo {
            path: path.to_string_lossy().to_string(),
            size_bytes,
            is_image: false,
            image_width: None,
            image_height: None,
            image_format: None,
            color_type: None,
        }),
    }
}