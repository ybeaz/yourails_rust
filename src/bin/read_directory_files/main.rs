use yourails_rust::shared::read_directory_files::read_directory_files;

// Run: cargo run --bin read_directory_files -- "/Users/admin/Dev/yourails_rust/src/bin/convert_img_pdf/__mocks__"
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::env::args()
        .nth(1)
        .expect("Usage: read_directory_files <directory>");

    let files = read_directory_files(&input, &["png", "jpg"])?;

    println!("Folder: {input}");
    for file in files {
        println!("{file}");
    }

    Ok(())
}
