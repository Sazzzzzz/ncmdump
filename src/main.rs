mod crypto;
mod ncm;

use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Collect arguments, skipping the binary name
    let args: Vec<String> = env::args().skip(1).collect();

    let mut files_to_process = Vec::new();

    if args.is_empty() {
        // Fallback 1: scan current working directory
        let mut target_dir = env::current_dir()?;

        let mut has_ncm_in_cwd = false;
        for entry in fs::read_dir(&target_dir)? {
            let path = entry?.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("ncm") {
                has_ncm_in_cwd = true;
                break;
            }
        }

        // Fallback 2: if CWD has no NCMs, use the executable's directory
        if !has_ncm_in_cwd
            && let Ok(exe_path) = env::current_exe()
            && let Some(parent) = exe_path.parent()
        {
            target_dir = parent.to_path_buf();
        }

        let entries = fs::read_dir(&target_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("ncm") {
                files_to_process.push(path);
            }
        }

        if files_to_process.is_empty() {
            println!("未在当前目录或可执行文件所在目录找到任何 .ncm 文件。");
            println!("使用方法：");
            println!("1. 将 .ncm 文件拖拽到此可执行文件上");
            println!("2. 双击运行该程序，解压当前目录的 .ncm 文件并解压。");
            println!("3. 通过命令行运行：ncmdump <file1.ncm> <file2.ncm> ...");
            return Ok(());
        }
    } else {
        for arg in args {
            let path = PathBuf::from(arg);
            if path.is_file() {
                files_to_process.push(path);
            } else {
                eprintln!("警告: {} 不是有效的文件。", path.display());
            }
        }
    }

    for path in files_to_process {
        println!("正在解压：{}", path.display());
        match ncm::dump(&path) {
            Ok(out_path) => println!("解压成功：{}", out_path.display()),
            Err(e) => eprintln!("解压失败：{}: {:?}", path.display(), e),
        }
    }

    Ok(())
}
