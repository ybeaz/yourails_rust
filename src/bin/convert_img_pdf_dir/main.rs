use std::path::Path;
use yourails_rust::shared::convert_image_to_pdf::convert_image_to_pdf;
use yourails_rust::shared::current_datetime_string::current_datetime_string;
use yourails_rust::shared::read_directory_files::read_directory_files;

// Run: cargo run --bin convert_img_pdf_dir -- "/Users/admin/Dev/yourails_rust/src/bin/convert_img_pdf/__mocks__"
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::env::args()
        .nth(1)
        .expect("Usage: read_directory_files <directory>");

    let files = read_directory_files(&input, &["png", "jpg"])?;

    println!("Folder: {input}");
    for file in files {
        let filename = Path::new(&file)
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("image");

        let timestamp: String = current_datetime_string();
        let output = format!("{input}/{timestamp}_{filename}.pdf");

        convert_image_to_pdf(&file, &output)?;
        println!("{timestamp} {file} -> {output}");
    }

    Ok(())
}
