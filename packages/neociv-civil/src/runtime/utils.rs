use std::path::Path;

pub fn read_entity_file(file: &str) -> Result<&str, ()> {
    // Confirm file exists
    if Path::new(file).exists() {
        Err(())
    } else {
        // Parse the json
        Ok("hello")
    }
}
