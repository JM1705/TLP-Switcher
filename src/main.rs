use std::fs;

fn main() {
    println!("TLP config switcher");
    fs::rename("~/Desktop/test/folder1/file1", "~/Desktop/test/folder1/file2")?;
}

