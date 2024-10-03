use crate::util;
use std::fs;
use std::path::Path;
use std::fmt::Display;

use clap::Parser;

/// 主程序运行
#[derive(Parser)]
pub struct App {
    /// 目录路径
    #[arg(short, long, value_name = "目录路径", help = "指定要读取的目录路径")]
    path: String,

    /// 需要的文件名
    #[arg(short, long, value_name = "需要的文件名", help = "指定需要读取的文件名（模糊匹配）")]
    file_names: Option<Vec<String>>,

    /// 不需要的路径
    #[arg(short, long, value_name = "不需要的路径", help = "指定不需要读取的路径（模糊匹配）")]
    not_need_paths: Option<Vec<String>>,
}

impl App {
    /// 程序启动入口
    pub fn run(&self) -> Result<(), String> {
        let mut file_infos = self.traverse_path_get_file_contents(&self.path)?;
        let mut all_contents = String::new();
        let mut file_count = 0;
        while let Some(file_info) = file_infos.pop() {
            all_contents.push_str(file_info.to_string().as_str());
            file_count += 1;
        }

        match util::write_to_clipboard(&all_contents).map_err(|e| e.to_string()) {
            Ok(_) => {
                println!("已将 {file_count} 个文件内容复制到剪贴板");
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    fn traverse_path_get_file_contents(&self, path: impl AsRef<Path>) -> Result<Vec<FileInfo>, String> {
        let mut results = Vec::new();
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                let mut new_results = self.traverse_path_get_file_contents(entry_path)?;
                results.append(&mut new_results);
            } else if entry_path.is_file() {
                let name = entry_path.file_name().unwrap().to_str().unwrap().to_owned();
                let display_path = entry_path.display().to_string();
                // 跳过非文本文件
                if !util::is_text_file(&entry_path).map_err(|e| e.to_string())? {
                    println!("跳过非文本文件: {name}");
                    continue;
                }

                // 过滤路径
                if let Some(not_need_paths) = &self.not_need_paths {
                    if not_need_paths.iter().any(|filter| display_path.contains(filter)) {
                        println!("跳过指定的路径: {display_path}");
                    }
                    continue;
                };

                // 过滤文件名
                if let Some(names) = &self.file_names {
                    if names.iter().all(|filter| !name.contains(filter)) {
                        println!("跳过非过滤列表文件: {name}");
                        continue;
                    }
                }
                let content = match fs::read_to_string(&entry_path) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("在读取 {name} 的内容时出错: {e}，跳过此文件");
                        continue
                    },
                };
                results.push(FileInfo { path: entry_path.display().to_string(), content })
            } 
        }
        Ok(results)
    }
}

/// 文件信息
pub struct FileInfo {
    /// 文件路径
    pub path: String,
    /// 文件内容
    pub content: String,
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "文件路径：\n{}\n文件内容：\n{}\n", self.path, self.content)
    }
}