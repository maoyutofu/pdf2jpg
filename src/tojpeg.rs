use pdfium_render::prelude::*;
use std::path::Path;

pub fn pdf_to_jpegs(input_path: &str, out_path: &str) -> Result<(), PdfiumError> {
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let document = pdfium.load_pdf_from_file(input_path, None)?;

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfBitmapRotation::Degrees180, true);

    let p = Path::new(input_path);
    let name = match p.file_name() {
        Some(name) => name.to_str().unwrap_or("unknown"),
        None => "unknown",
    };
    let (name, _) = name.rsplit_once('.').unwrap_or(("unknown", ""));
    for (index, page) in document.pages().iter().enumerate() {
        page.render_with_config(&render_config)?
            .as_image()
            .as_rgba8()
            .ok_or(PdfiumError::ImageError)?
            .save_with_format(
                format!("{}/{}-{}.jpg", out_path, name, index),
                image::ImageFormat::Jpeg,
            )
            .map_err(|_| PdfiumError::ImageError)?;
    }
    Ok(())
}

