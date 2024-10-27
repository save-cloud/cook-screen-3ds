//! fork from https://github.com/sweihub/log2
//! - delete color
//! - remove chrono
//! - mask key info
//!
//!# log2
//!
//!`log2` is an out-of-the-box logging library for Rust. It writes to stdout or to file asynchronousely,
//!and automatically rotates based on file size.
//!
//!# Usage
//!
//!## Add dependency
//!```
//!cargo add log2
//!```
//!
//!## Log to stdout
//!
//!Simple to start.
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!let _log2 = log2::start();
//!
//!trace!("send order request to server");
//!debug!("receive order response");
//!info!("order was executed");
//!warn!("network speed is slow");
//!error!("network connection was broken");
//!}
//!```
//!
//!Output
//!
//!![Screnshot of log2 output](images/output.png)
//!
//!Show module path, and set log level.
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!let _log2 = log2::stdout()
//!.module(true)
//!.level("info")
//!.start();
//!
//!trace!("send order request to server");
//!debug!("receive order response");
//!info!("order was executed");
//!warn!("network speed is slow");
//!error!("network connection was broken");
//!}
//!
//!```
//!
//!## Log to file
//!
//!`log2` with default file size 100MB, max file count 10, you can change as you like. Note the `_log2` will
//!stop the log2 instance when it is out of the scope
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!// configurable way:
//!// - log to file, file size: 100 MB, rotate: 20
//!// - tee to stdout
//!// - show module path
//!let _log2 = log2::open("log.txt")
//!.size(100*1024*1024)
//!.rotate(20)
//!.tee(true)
//!.module(true)
//!.start();
//!
//!// out-of-the-box way
//!// let _log2 = log2::open("log.txt").start();
//!
//!trace!("send order request to server");
//!debug!("receive order response");
//!info!("order was executed");
//!warn!("network speed is slow");
//!error!("network connection was broken");
//!}
//!
//!```
//!
//!Output files
//!
//!```
//!log.txt
//!log.1.txt
//!log.2.txt
//!log.3.txt
//!log.4.txt
//!log.5.txt
//!log.6.txt
//!log.7.txt
//!log.8.txt
//!log.9.txt
//!```
use core::fmt;
use log::{Level, LevelFilter, Metadata, Record};
use std::{io::Write, path::Path};

/// log macros
pub use log::{debug, error, info, trace, warn};

use crate::utils::get_current_format_time;

/// log levels
#[allow(non_camel_case_types)]
pub type level = LevelFilter;

fn get_level(level: String) -> LevelFilter {
    let level = level.to_lowercase();
    match &*level {
        "debug" => level::Debug,
        "trace" => level::Trace,
        "info" => level::Info,
        "warn" => level::Warn,
        "error" => level::Error,
        "off" => level::Off,
        _ => level::Debug,
    }
}

/// set the log level, the input can be both enum or name
pub fn set_level<T: fmt::Display>(level: T) {
    log::set_max_level(get_level(level.to_string()));
}

pub struct Log2 {
    path: String,
    tee: bool,
    module: bool,
    file_size: u64,
    count: usize,
    level: String,
    mask: Vec<&'static str>,
    lock: std::sync::Mutex<()>,
}

impl Log2 {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            tee: false,
            module: true,
            file_size: 100 * 1024 * 1024,
            count: 10,
            level: String::new(),
            mask: vec![],
            lock: std::sync::Mutex::new(()),
        }
    }

    pub fn module(mut self, show: bool) -> Log2 {
        self.module = show;
        self
    }

    // split the output to stdout
    pub fn tee(mut self, stdout: bool) -> Log2 {
        self.tee = stdout;
        self
    }

    /// setup the maximum size for each file
    pub fn size(mut self, file_size: u64) -> Log2 {
        if self.count <= 1 {
            self.file_size = std::u64::MAX;
        } else {
            self.file_size = file_size;
        }
        self
    }

    /// setup the rotate count
    pub fn rotate(mut self, count: usize) -> Log2 {
        self.count = count;
        if self.count <= 1 {
            self.file_size = std::u64::MAX;
        }
        self
    }

    pub fn level<T: fmt::Display>(mut self, name: T) -> Self {
        self.level = name.to_string();
        self
    }

    pub fn mask(mut self, mask: Vec<&'static str>) -> Self {
        self.mask = mask;
        self
    }

    /// start the log2 instance
    pub fn start(self) -> bool {
        let tee = self.tee;
        let level = self.level.clone();

        log::set_boxed_logger(Box::new(self)).expect("log2 error initialize failed");
        log::set_max_level(LevelFilter::Trace);

        if !level.is_empty() {
            set_level(level);
        }

        tee
    }

    pub fn do_write_line_to_file(&self, line: String) {
        let _lock = self.lock.lock().expect("log2 error get lock");
        let path = self.path.clone();
        let file_size = self.file_size;
        let count = self.count;
        let mask = self.mask.clone();
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&path)
        {
            // mask key info
            let line = if let Some(index) = mask.iter().find_map(|s| line.find(s)) {
                let mut line = line;
                let max_count = 60;
                for idx in 0..max_count {
                    if index + 13 + max_count - idx < line.len() {
                        line = format!(
                            "{}{}{}",
                            &line[..index + 13],
                            "******",
                            &line[index + 13 + max_count - idx..]
                        );
                        break;
                    }
                }
                line
            } else {
                line
            };
            file.write_all(line.as_bytes()).ok();
            if let Some(current_size) = file.metadata().ok().map(|m| m.len()) {
                drop(file);
                if current_size >= file_size {
                    Log2::do_rotate(&path, count).ok();
                }
            }
        }
    }

    pub fn do_rotate(path: &str, count: usize) -> Result<(), std::io::Error> {
        let dot = path.rfind(".").unwrap_or(0);
        let mut suffix = "";
        let mut prefix = &path[..];
        if dot > 0 {
            suffix = &path[dot..];
            prefix = &path[0..dot];
        }

        for i in (0..count - 1).rev() {
            let mut a = format!("{prefix}.{}{suffix}", i);
            if i == 0 {
                a = path.to_string();
            }
            let b = format!("{prefix}.{}{suffix}", i + 1);
            if Path::new(&a).exists() {
                if Path::new(&b).exists() {
                    let _ = std::fs::remove_file(&b);
                }
                let _ = std::fs::rename(&a, &b);
            }
        }

        Ok(())
    }
}

unsafe impl Sync for Log2 {}

impl log::Log for Log2 {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // this seems no effect at all
        metadata.level() >= Level::Error
    }

    fn log(&self, record: &Record) {
        // cheap way to ignore other crates with absolute files (UNIX)
        // TODO: filter by crate/module name?
        let file = record.file().unwrap_or("unknown");
        if file.starts_with("/") {
            return;
        }

        let mut module = "".into();
        if self.module {
            if file.starts_with("src/") && file.ends_with(".rs") {
                module = format!("{}: ", &file[4..file.len() - 3]);
            } else {
                module = format!("{file}: ");
            }
        }

        if self.tee || self.path.len() > 0 {
            let line = format!(
                "[{}] [{}] {module}{}\n",
                &get_current_format_time(),
                record.level(),
                record.args()
            );

            // stdout
            if self.tee {
                print!("{}", line);
            }

            // file
            if self.path.len() > 0 {
                self.do_write_line_to_file(line);
            }
        }
    }

    fn flush(&self) {
        //
    }
}

/// log to file
pub fn open(path: &str) -> Log2 {
    // create directory
    let dir = std::path::Path::new(path);
    if let Some(dir) = dir.parent() {
        let _ = std::fs::create_dir_all(&dir);
    }

    let mut logger = Log2::new();
    logger.path = path.into();
    logger
}
