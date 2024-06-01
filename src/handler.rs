#[allow(unused_imports)]
use std::fs::{File, copy, create_dir_all};
// use std::io::Write;

#[allow(dead_code)]
pub fn write_brv() {
    // ...
}

#[allow(dead_code)]
pub fn write_preview(project_path: &str) -> std::io::Result<()> {
    // We need to clone BRCI_preview.png.
    copy("BRCI_preview.png", format!("{}/Preview.png", project_path))?;
    Ok(())
}

#[allow(dead_code)]
pub fn make_project(project_name: &str, project_directory: &str) -> std::io::Result<()> {
    let project_path = format!("{}/{}", project_directory, project_name);
    create_dir_all(&project_path)?;
    write_brv();
    write_preview(&project_path)?;
    Ok(())
}