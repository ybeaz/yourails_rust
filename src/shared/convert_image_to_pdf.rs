use ::image::{ImageFormat, ImageReader};
use printpdf::{Mm, Op, PdfDocument, PdfPage, PdfSaveOptions, Pt, RawImage, XObjectTransform};
use std::fs::File;
use std::io::{BufWriter, Cursor, Write};

pub fn convert_image_to_pdf(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read and decode the source image.
    let image = ImageReader::open(input)?.decode()?;
    let (width_px, height_px) = (image.width(), image.height());

    // Convert the image to PNG bytes in memory.
    let mut image_bytes = Vec::new();
    image.write_to(&mut Cursor::new(&mut image_bytes), ImageFormat::Png)?;

    // Decode the PNG bytes into printpdf's RawImage.
    let mut warnings = Vec::new();
    let raw_image = RawImage::decode_from_bytes(&image_bytes, &mut warnings)?;

    // Create PDF document.
    let mut document = PdfDocument::new("Image to PDF");
    let image_id = document.add_image(&raw_image);

    // ---------------------------------------------------------
    // A4 portrait page
    // ---------------------------------------------------------
    const A4_WIDTH_MM: f32 = 210.0;
    const A4_HEIGHT_MM: f32 = 297.0;
    const POINTS_PER_MM: f32 = 72.0 / 25.4;
    let page_width_pt = A4_WIDTH_MM * POINTS_PER_MM;
    let page_height_pt = A4_HEIGHT_MM * POINTS_PER_MM;

    // ---------------------------------------------------------
    // Image dimensions
    // ---------------------------------------------------------
    // printpdf places images at "1 pixel = 1 dot at 300 dpi" when
    // scale_x/scale_y == 1.0 — i.e. the image's *native* size in
    // points is (pixels * 72 / 300), NOT (pixels * 1). scale_x/scale_y
    // are multipliers on top of that native size, not on raw pixels.
    const NATIVE_DPI: f32 = 300.0;
    let native_width_pt = width_px as f32 * 72.0 / NATIVE_DPI;
    let native_height_pt = height_px as f32 * 72.0 / NATIVE_DPI;

    // Scale so the image's rendered width exactly matches the A4 width.
    // Aspect ratio is preserved because scale_x == scale_y.
    let scale = page_width_pt / native_width_pt;
    let scaled_width_pt = native_width_pt * scale; // == page_width_pt
    let scaled_height_pt = native_height_pt * scale;

    // Center horizontally (will be ~0 since scaled_width_pt == page_width_pt).
    let translate_x_pt = (page_width_pt - scaled_width_pt) / 2.0;
    // Center vertically.
    let translate_y_pt = (page_height_pt - scaled_height_pt) / 2.0;

    // ---------------------------------------------------------
    // Create A4 page with centered image
    // ---------------------------------------------------------
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

    // ---------------------------------------------------------
    // Save PDF
    // ---------------------------------------------------------
    let pdf_bytes = document.save(&PdfSaveOptions::default(), &mut warnings);
    let file = File::create(output)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&pdf_bytes)?;
    Ok(())
}
