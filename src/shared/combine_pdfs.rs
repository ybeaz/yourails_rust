use printpdf::image_types::ImageOptimizationOptions;
use printpdf::{PdfDocument, PdfSaveOptions, deserialize::PdfParseOptions};
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn combine_pdfs(
    input_paths: &Vec<String>,
    output_path_abs: &str,
    max_image_size_bytes: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut warnings = Vec::new();
    let parse_options = PdfParseOptions::default();

    let mut input_paths_iter = input_paths.into_iter();

    let first_path = input_paths_iter
        .next()
        .ok_or("combine_pdfs: input_paths is empty")?;

    let first_bytes = std::fs::read(&first_path)?;
    let mut merged_document = PdfDocument::parse(&first_bytes, &parse_options, &mut warnings)
        .map_err(|e| format!("failed to parse {first_path}: {e}"))?;

    for path in input_paths_iter {
        let bytes = std::fs::read(&path)?;
        let doc = PdfDocument::parse(&bytes, &parse_options, &mut warnings)
            .map_err(|e| format!("failed to parse {path}: {e}"))?;
        merged_document.append_document(doc);
    }

    let max_image_size_mb = max_image_size_bytes / 100_000;

    let save_options = PdfSaveOptions {
        image_optimization: Some(ImageOptimizationOptions {
            max_image_size: Some(format!("{max_image_size_mb}mb")),
            auto_optimize: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    };
    let pdf_bytes = merged_document.save(&save_options, &mut warnings);

    let file = File::create(output_path_abs)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&pdf_bytes)?;

    Ok(())
}
