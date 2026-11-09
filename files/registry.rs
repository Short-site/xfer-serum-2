//! Windows registry read helpers for Xfer Serum 2.

pub const APP_KEY: &str = "Software\\Xfer\\Xfer Serum 2";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Xfer Serum 2";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Xfer Serum 2") },
        RegValue { name: "Version",     value: String::from("v4.5.23") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Xfer Serum 2") },
        RegValue { name: "Publisher",       value: String::from("Xfer") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Xfer Serum 2\\uninstall.exe\"") },
    ]
}
