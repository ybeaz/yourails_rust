use crate::shared::combine_pdfs::combine_pdfs;
use crate::shared::convert_image_to_pdf::convert_image_to_pdf;
use crate::shared::delete_files::delete_files;
use std::path::Path;

pub struct ConvertImgPdPathsArgs {
    pub path_dir_abs: String,
    pub timestamp: String,
    pub index_group: usize,
    pub paths_files_abs: Vec<String>,
}

pub fn convert_imgs_paths_to_1pdf(
    ConvertImgPdPathsArgs {
        path_dir_abs,
        timestamp,
        index_group,
        paths_files_abs,
    }: ConvertImgPdPathsArgs,
) -> Result<String, String> {
    let mut files_output: Vec<String> = Vec::new();

    for (_, path_file_abs) in paths_files_abs.iter().enumerate() {
        let filename = Path::new(&path_file_abs)
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("image");

        let path_file_abs_item_output =
            format!("{path_dir_abs}/{timestamp}_{index_group}_{filename}.pdf");
        let _ = convert_image_to_pdf(&path_file_abs, &path_file_abs_item_output, 5.0, 1_000_000);

        files_output.push(path_file_abs_item_output);
    }

    let path_file_abs_output = format!("{path_dir_abs}/{timestamp}_{index_group}.pdf");
    let _ = combine_pdfs(
        &files_output,
        &path_file_abs_output,
        1_000_000 * files_output.len() as u64,
    );

    let _ = delete_files(&files_output);

    Ok(path_file_abs_output)
}
