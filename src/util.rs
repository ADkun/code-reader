#![allow(dead_code)]

use clipboard::{ClipboardContext, ClipboardProvider};
use std::env;
use std::path::Path;
use std::io::{self, Read};
use std::fs::File;

/// 向系统剪贴板写入文本
pub fn write_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(text.to_owned())?;
    Ok(())
}

/// 获取命令行参数
pub fn env_args() -> Vec<String> {
    env::args().collect()
}

/// 判断路径是否存在且是目录
pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

/// 判断文件是否是文本文件
pub fn is_text_file<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let mut buffer = vec![0; 1024]; // 读取前1024字节
    let bytes_read = file.read(&mut buffer)?;

    // 尝试将读取的字节解析为 UTF-8
    match std::str::from_utf8(&buffer[..bytes_read]) {
        Ok(_) => Ok(true),  // 如果能够解析为 UTF-8，则认为是文本文件
        Err(_) => Ok(false),  // 否则认为是二进制文件
    }
}