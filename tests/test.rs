use std::process::Command;

#[tokio::test]
async fn file_exists_for_every_line() {
    let list = list();
    for line in list {
        println!("{}", line);
    }
}

fn list() -> Vec<String> {
    let out = Command::new("./target/debug/cargo-listdoc")
        .args(&["listdoc", "show"])
        .output()
        .unwrap();
    let out = String::from_utf8(out.stdout).unwrap();
    lines(out)
}

fn lines(s: String) -> Vec<String> {
    let mut lines = Vec::new();
    let mut buf = s;
    while let Some(idx) = buf.find('\n') {
        let new = buf.split_off(idx);
        lines.push(buf);
        buf = new;
    }
    if !buf.is_empty() {
        lines.push(buf);
    }
    lines
}
