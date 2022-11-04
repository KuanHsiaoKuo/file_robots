extern crate chrono;

#[cfg(windows)]
use std::ffi::CString;

use chrono::Local;
use directories::ProjectDirs;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::{LevelFilter, info};

pub fn subs(str: &str) -> Vec<String> {
    if let Ok(paths) = std::fs::read_dir(str) {
        return paths
            .into_iter()
            .map(|x| x.unwrap().path().to_str().unwrap().to_string())
            .collect();
    }
    vec![]
}

pub fn open_file_path(path: &str) {
    let curr_path = std::path::Path::new(path);
    let arg;
    if curr_path.is_dir() {
        arg = curr_path.to_str().unwrap();
    } else {
        arg = curr_path.parent().unwrap().to_str().unwrap();
    }

    if cfg!(target_os = "windows") {
        std::process::Command::new("explorer")
            .args([win_norm4explorer(arg)])
            .output()
            .expect("failed to execute process");
    } else if cfg!(target_os = "linux") {
        std::process::Command::new("xdg-open")
            .args([arg])
            .output()
            .expect("failed to execute process");
    } else {
        //mac os
        std::process::Command::new("open")
            .args([arg])
            .output()
            .expect("failed to execute process");
    }
}

pub fn excel_automation_backend(path: &str) {
    let curr_path = std::path::Path::new(path);
    let arg;
    if curr_path.is_dir() {
        arg = curr_path.to_str().unwrap();
    } else {
        arg = curr_path.parent().unwrap().to_str().unwrap();
    }
    println!("excel_automation_backend: {}", arg);
}

pub fn open_file_path_in_terminal(path: &str) {
    let curr_path = std::path::Path::new(path);
    let arg;
    if curr_path.is_dir() {
        arg = curr_path.to_str().unwrap();
    } else {
        arg = curr_path.parent().unwrap().to_str().unwrap();
    }
    println!("open_file_path_in_terminal: {}", arg);

    if cfg!(target_os = "windows") {
        //cmd /K "cd C:\Windows\"
        std::process::Command::new("cmd")
            .args([
                "/c",
                "start",
                "cmd",
                "/K",
                "pushd",
                &format!("{}", win_norm4explorer(arg)),
            ])
            .output()
            .expect("failed to execute process");
    } else if cfg!(target_os = "linux") {
        // gnome-terminal -e "bash -c command;bash"
        std::process::Command::new("gnome-terminal")
            .args(["-e", &format!("bash -c 'cd {}';bash", arg)])
            .output()
            .expect("failed to execute process");
    } else {
        //mac os
        //open -a Terminal "/Library"
        std::process::Command::new("open")
            .args(["-a", "Terminal", arg])
            .output()
            .expect("failed to execute process");
    }
}

pub fn data_dir() -> String {
    let project_dir = ProjectDirs::from("com", "github", "Orange").unwrap();
    project_dir.data_dir().to_str().unwrap().to_string()
}

pub fn path2name(x: String) -> String {
    norm(&x)
        .as_str()
        .split("/")
        .into_iter()
        .last()
        .map(|x| x.to_string())
        .unwrap_or("".to_string())
}

pub fn file_ext(file_name: &str) -> &str {
    if !file_name.contains(".") {
        return "";
    }
    file_name.split(".").last().unwrap_or("")
}

pub fn norm(path: &str) -> String {
    str::replace(path, "\\", "/")
}

pub fn today() -> String {
    let date = Local::now();
    date.format("%Y-%m-%d").to_string()
}

pub fn win_norm4explorer(path: &str) -> String {
    str::replace(path, "/", "\\")
}

#[cfg(windows)]
pub unsafe fn get_win32_ready_drives() -> Vec<String> {
    let mut logical_drives = Vec::with_capacity(5);
    let mut bitfield = kernel32::GetLogicalDrives();
    let mut drive = 'A';

    while bitfield != 0 {
        if bitfield & 1 == 1 {
            let strfulldl = drive.to_string() + ":/";
            let cstrfulldl = CString::new(strfulldl.clone()).unwrap();
            let x = kernel32::GetDriveTypeA(cstrfulldl.as_ptr());
            if x == 3 || x == 2 {
                logical_drives.push(strfulldl);
                // println!("drive {0} is {1}", strfdl, x);
            }
        }
        drive = std::char::from_u32((drive as u32) + 1).unwrap();
        bitfield >>= 1;
    }
    logical_drives.sort_by(|x1, x2| x2.cmp(x1));
    logical_drives
}

pub fn is_ascii_alphanumeric(raw: &str) -> bool {
    raw.chars().all(|x| x.is_ascii())
}

pub fn init_log() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} -{t} - {m}{n}")))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {t} - {m}{n}")))
        .build(format!("{}/log/{}.log", data_dir(), today()))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(
            Logger::builder()
                .appender("file")
                .appender("stdout")
                .additive(false)
                .build("app", LevelFilter::Info),
        )
        .build(Root::builder().appender("stdout").build(LevelFilter::Error))
        .unwrap();

    let _ = log4rs::init_config(config).unwrap();
}

#[cfg(windows)]
pub unsafe fn get_win32_ready_drive_nos() -> Vec<String> {
    let vec = get_win32_ready_drives();
    let mut res = vec![];
    for x in vec {
        let s = str::replace(x.as_str(), ":/", "");
        res.push(s);
    }
    res.sort();
    res
}

pub fn win_norm4exclude_path(x: String) -> String {
    let (x1, x2) = x.split_at(1);
    let mut up = x1.to_uppercase();
    up.push_str(x2);
    up.replace("//", "/")
}

#[cfg(windows)]
pub unsafe fn build_volume_path(str: &str) -> String {
    str::replace("\\\\?\\$:", "$", str)
}

#[cfg(test)]
mod tests {
    use convert_case::{Case, Casing};

    use super::*;

    #[cfg(windows)]
    #[test]
    fn t1() {
        let str = "c";
        let string = unsafe { build_volume_path(str) };
        println!("{}", string);
    }

    #[test]
    fn t2() {
        println!("{}", data_dir());
    }

    #[test]
    fn t3() {
        let chines = is_ascii_alphanumeric("j dsadal");
        println!("{:?}", chines);
    }

    #[test]
    fn t4() {
        open_file_path_in_terminal("/home/jeff/CLionProjects/orange")
        // use std::process::Command;
        // Command::new("cmd")
        //     .args(&["/c", "start", "cmd"])
        //     .spawn()
        //     .unwrap();
    }

    #[test]
    fn t5() {
        let ext = file_ext("java");
        println!("{}", ext);
    }

    #[test]
    fn t6() {
        let string = "FilterFieldNamesProvidingStoredFieldsVisitor.java".to_case(Case::Title);
        println!("{}", string);
    }

    #[test]
    fn t7() {
        let _result = webbrowser::open("https://github.com/naaive/orange/releases");
    }

    #[test]
    fn t8() {
        let x = "c://".to_string();

        let string = win_norm4exclude_path(x);

        println!("{}", string);
    }
}
