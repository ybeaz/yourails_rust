// use crate::shared::get_file_info::get_file_info;
use printpdf::image_types::ImageOptimizationOptions;
use printpdf::{Mm, Op, PdfDocument, PdfPage, PdfSaveOptions, Pt, RawImage, XObjectTransform};
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn convert_image_to_pdf(
    input: &str,
    output: &str,
    max_scale: f32,
    max_image_size_bytes: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // let file_info = get_file_info(&input)?;

    let image_bytes = std::fs::read(input)?;

    let mut warnings = Vec::new();

    let raw_image = RawImage::decode_from_bytes(&image_bytes, &mut warnings)?;

    let (width_px, height_px) = (raw_image.width, raw_image.height);

    let mut document = PdfDocument::new("Image to PDF");

    // println!(
    //     "convert_image_to_pdf [90] RawImage: {}x{}",
    //     raw_image.width, raw_image.height,
    // );

    let image_id = document.add_image(&raw_image);

    const A4_WIDTH_MM: f32 = 210.0;
    const A4_HEIGHT_MM: f32 = 297.0;
    const POINTS_PER_MM: f32 = 72.0 / 25.4;
    let page_width_pt = A4_WIDTH_MM * POINTS_PER_MM;
    let page_height_pt = A4_HEIGHT_MM * POINTS_PER_MM;

    const NATIVE_DPI: f32 = 300.0;
    let native_width_pt = width_px as f32 * 72.0 / NATIVE_DPI;
    let native_height_pt = height_px as f32 * 72.0 / NATIVE_DPI;

    let fit_width_scale = page_width_pt / native_width_pt;
    let fit_height_scale = page_height_pt / native_height_pt;
    let scale = fit_width_scale.min(fit_height_scale).min(max_scale);

    let scaled_width_pt = native_width_pt * scale;
    let scaled_height_pt = native_height_pt * scale;

    let translate_x_pt = (page_width_pt - scaled_width_pt) / 2.0;
    let translate_y_pt = (page_height_pt - scaled_height_pt) / 2.0;

    let page = PdfPage::new(
        Mm(A4_WIDTH_MM),
        Mm(A4_HEIGHT_MM),
        vec![Op::UseXobject {
            id: image_id,
            transform: XObjectTransform {
                translate_x: Some(Pt(translate_x_pt)),
                translate_y: Some(Pt(translate_y_pt)),
                scale_x: Some(scale),
                scale_y: Some(scale),
                ..Default::default()
            },
        }],
    );

    document.with_pages(vec![page]);

    let max_image_size_mb = max_image_size_bytes / 100_000;

    let save_options = PdfSaveOptions {
        image_optimization: Some(ImageOptimizationOptions {
            max_image_size: Some(format!("{max_image_size_mb}mb")),
            auto_optimize: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    };

    let pdf_bytes = document.save(&save_options, &mut warnings);

    let file = File::create(output)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&pdf_bytes)?;

    // let file_info2 = get_file_info(&output)?;
    // println!(
    //     "\nconvert_image_to_pdf [160] {output} {} > {} bytes\n",
    //     file_info.size_bytes, file_info2.size_bytes,
    // );

    Ok(())
}
