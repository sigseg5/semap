use std::process::Command;

pub fn set(key: &str, value: &str) -> Result<(), String> {
    let mut cmd = Command::new("dconf");
    cmd.args(&["write", key, value]);
    match cmd.output() {
        Ok(_) => Ok(()),
        Err(_) => Err("Unable to set key".to_string()),
    }
}
