mod handler;
#[allow(unused_imports)]
use to_binary::{BinaryString,BinaryError};
#[macro_use] mod misc;
use misc::FM; // The glob operator can be used but is dangerous if you do not know what you are doing.

const VERSION: &str = "V0.3.7";

/* Semantic versioning.
Scheme goes like this:

First number: Incremented when a massive update is pushed, making previous API or interface incompatible
Second number: Major update, program may be significantly changed
Third number: Smaller update, usually a hotfix or patch */

fn main() {
    printlnr!("\n✨ {}Welcome to BRCI.rs!", FM::light_green);
    printlnr!("📋 {}Program version: [{VERSION}]\n", FM::light_blue);
    
    // It is my belief that this structure shouldn't be used. Not in the current state.
    /* #[allow(dead_code)]
    struct Colors {

    }

    #[allow(dead_code)]
    struct ErrorHandler {

    }

    #[allow(dead_code)]
    pub struct Vehicle {
        
    }
    */
    let project_name: &str = "Test";
    let project_dir: &str = "Projects";
    let make_proj_result = match handler::make_project(project_name, project_dir) {
        Ok(_) => "Success".to_string(),
        Err(val) => val.to_string(),
    };

    if make_proj_result == "Success" {
        printlnr!("✅ {}Project created successfully!", FM::light_green);
    } else {
        printlnr!("❌ {}Failed to create project!\n'{}'", FM::light_red, make_proj_result);
    }
}
