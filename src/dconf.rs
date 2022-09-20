// Inspired by https://github.com/kylecorry31/dconf_rs

use std::process::Command;
use std::process::Output;

#[doc = "This function can set any `value` for dconf `key`."]
pub fn set(key: &str, value: &str) -> Result<(), String> {
    let mut cmd = Command::new("dconf");
    cmd.args(&["write", key, value]);
    match cmd.output() {
        Ok(_) => Ok(()),
        Err(_) => Err("Unable to set key".to_string()),
    }
}

#[doc = "This function can read value for any dconf `key`."]
pub fn get(key: &str) -> Result<String, String> {
    let mut cmd = Command::new("dconf");
    cmd.args(&["read", key]);
    match cmd.output() {
        Ok(output) => Ok(get_stdout(output)),
        Err(_) => Err("Unable to get key".to_string()),
    }
}

#[doc = "This function makes some output formatting operations (like replacing escape codes)."]
fn get_stdout(output: Output) -> String {
    let vs = output.stdout;
    String::from_utf8(vs)
        .unwrap()
        .replace("\'", "")
        .replace("\n", "")
}
