#[allow(unused_imports)]
use std::fs::{File, copy, create_dir_all, remove_file};
use std::path::Path;

use crate::misc::FM;
// tell rust that 'misc' is in src/misc/mod.rs

// use std::io::Write;

#[allow(unused_variables)]
pub fn write_brv(project_path: &str) -> std::io::Result<()> {
    // ...
    Ok(())
}

#[allow(unused_variables)]
pub fn write_preview(project_path: &str) -> std::io::Result<()> {
    // We need to clone BRCI_preview.png.
    copy("BRCI_preview.png", format!("{}/Preview.png", project_path))?;
    Ok(())
}

#[allow(unused_variables)]
pub fn write_brm(project_path: &str) -> std::io::Result<()> {
    // ...
    Ok(())
}

#[allow(dead_code)]
pub fn make_project(project_name: &str, project_directory: &str) -> std::io::Result<()> {
    let project_path: String = format!("{}/{}", project_directory, project_name);
    if Path::new(&project_path).exists() {
        // Remove the important files for an overwrite
        println!("📝 {}Found that project already exists. Overwriting it.{}", FM::light_blue, FM::reset);
        if Path::new(&format!("{}/Preview.png", project_path)).exists() { remove_file(format!("{}/Preview.png", project_path))?; }
        if Path::new(&format!("{}/Vehicle.brv", project_path)).exists() { remove_file(format!("{}/Vehicle.brv", project_path))?; }
        if Path::new(&format!("{}/Vehicle.brm", project_path)).exists() { remove_file(format!("{}/Vehicle.brm", project_path))?; }
    } else { // Directory doesn't already exist, create it
        create_dir_all(&project_path)?;
    }
    write_brv(&project_path)?;
    write_brm(&project_path)?;
    write_preview(&project_path)?;
    Ok(())
}