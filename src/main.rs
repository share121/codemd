use arboard::Clipboard;
use fs_err as fs;
use path_clean::PathClean;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let exts = args.next().expect("你需要传入类似 js,rs,html 的扩展名列表");
    let exts: Vec<_> = exts.split(",").map(|s| s.trim()).collect();
    let current_dir = env::current_dir().unwrap();
    let mut entries = Vec::new();
    for arg in args {
        entries.push(current_dir.join(arg));
    }
    if entries.is_empty() {
        entries.push(current_dir.clone());
    }
    let relative_dir = current_dir.parent().unwrap_or(&current_dir).to_path_buf();
    let mut md = String::new();
    let mut count = 0;
    while let Some(entry) = entries.pop() {
        if entry.is_file() {
            let ext = entry
                .extension()
                .map(|e| e.to_string_lossy())
                .unwrap_or_default();
            if !exts.contains(&ext.as_ref()) {
                continue;
            }
            let relative_path = entry.strip_prefix(&relative_dir).unwrap_or(&entry).clean();
            println!("发现 {}", relative_path.display());
            md += &format!(
                "```{} path=\"{}\"\n{}\n```\n\n",
                ext,
                relative_path.display(),
                fs::read_to_string(&entry).unwrap().trim_end()
            );
            count += 1;
        } else if entry.is_dir() {
            let Ok(dir) = fs::read_dir(entry).inspect_err(|e| eprintln!("{e}")) else {
                continue;
            };
            entries.extend(dir.flatten().map(|e| e.path()));
        }
    }
    println!("共生成 {} 个代码块", count);
    if Clipboard::new()
        .and_then(|mut c| c.set_text(&md))
        .inspect_err(|e| eprintln!("写入剪贴板失败: {e}"))
        .is_ok()
    {
        println!("已将代码块复制到剪贴板");
    } else {
        fs::write(current_dir.join("codemd.md"), &md).unwrap();
    }
}
