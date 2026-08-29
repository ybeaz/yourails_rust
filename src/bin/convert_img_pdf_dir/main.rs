use yourails_rust::shared::convert_imgs_paths_to_1pdf::ConvertImgPdPathsArgs;
use yourails_rust::shared::convert_imgs_paths_to_1pdf::convert_imgs_paths_to_1pdf;
use yourails_rust::shared::current_datetime_string::current_datetime_string;
use yourails_rust::shared::get_file_info::get_file_info;
use yourails_rust::shared::read_directory_files::read_directory_files;
use yourails_rust::shared::vecs2_from_vec_paths::vecs2_from_vec_paths;

// Run: cargo run --bin convert_img_pdf_dir -- "/Users/admin/Dev/yourails_rust/src/bin/convert_img_pdf_dir/__mocks__"
// To run globally
// 1.: cargo install --path . --bin convert_img_pdf_dir
// 2.: convert_img_pdf_dir "/Users/admin/Dev/yourails_rust/src/bin/convert_img_pdf_dir/__mocks__"
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path_dir_abs = std::env::args()
        .nth(1)
        .expect("Usage: read_directory_files <directory>");

    let paths_all = read_directory_files(&path_dir_abs, &["png", "jpg", "jpeg"])?;

    let timestamp: String = current_datetime_string();

    let paths_vecs2 = vecs2_from_vec_paths(&paths_all);

    println!("\nconvert_img_pdf_dir::main [30]");

    for (index_group, paths_files_abs) in paths_vecs2.iter().enumerate() {
        /* CONVERT VEC OF FILES INTO ONE PDF AND SAVE IT */
        let path_file_abs_output = convert_imgs_paths_to_1pdf(ConvertImgPdPathsArgs {
            path_dir_abs: path_dir_abs.clone(),
            timestamp: timestamp.clone(),
            index_group,
            paths_files_abs: paths_files_abs.clone(),
        })?;

        let file_info = get_file_info(&path_file_abs_output)?;

        println!("Group {}:", index_group + 1);
        for item in paths_files_abs {
            println!("  - {item}");
        }
        println!(
            "  > {path_file_abs_output} -> {} bytes\n",
            file_info.size_bytes,
        );
    }

    Ok(())
}
