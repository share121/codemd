use arboard::Clipboard;
use fs_err as fs;
use path_clean::PathClean;
use std::env;

fn main() {
    let mut clipboard = Clipboard::new().expect("无法获取剪贴板");
    let mut args = env::args();
    args.next();
    let exts = args.next().expect("你需要传入类似 js,rs,html 的扩展名列表");
    let exts: Vec<_> = exts.split(",").map(|s| s.trim()).collect();
    let current_dir = env::current_dir().unwrap();
    let mut dirs = Vec::new();
    for arg in args {
        dirs.push(current_dir.join(arg));
    }
    if dirs.is_empty() {
        dirs.push(current_dir.clone());
    }
    let relative_dir = current_dir.parent().unwrap_or(&current_dir).to_path_buf();
    let mut md = String::new();
    let mut count = 0;
    while let Some(dir) = dirs.pop() {
        let entries = fs::read_dir(dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_type = entry.file_type().unwrap();
            if file_type.is_dir() {
                dirs.push(path);
                continue;
            }
            let ext = path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            if !exts.contains(&ext) {
                continue;
            }
            let relative_path = path.strip_prefix(&relative_dir).unwrap_or(&path).clean();
            println!("发现 {}", relative_path.display());
            md += &format!(
                "```{} title=\"{}\"\n{}\n```\n\n",
                ext,
                relative_path.display(),
                fs::read_to_string(&path).unwrap().trim_end()
            );
            count += 1;
        }
    }
    println!("共生成 {} 个代码块", count);
    clipboard.set_text(md).expect("写入剪贴板失败");
    println!("已将代码块复制到剪贴板");
}
