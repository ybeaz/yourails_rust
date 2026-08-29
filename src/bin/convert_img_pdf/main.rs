use yourails_rust::shared::convert_image_to_pdf::convert_image_to_pdf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::env::args()
        .nth(1)
        .expect("Usage: convert_img_pdf <input-image> [output.pdf]");

    let output = std::env::args()
        .nth(2)
        .unwrap_or_else(|| format!("{input}.pdf"));

    convert_image_to_pdf(&input, &output, 5.0, 1_000_000)?;

    println!("Created: {output}");

    Ok(())
}
