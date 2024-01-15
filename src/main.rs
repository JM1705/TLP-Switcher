use std::fs;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() {
    movefile("file1.txt", "file2.txt")
}

fn movefile(start: &str, end: &str) {
    println!("TLP config switcher");

    let dir = "./";

    let mut dir_files = fs::read_dir(dir)
      .expect("failed to read directory")
        .map(|file| file.unwrap().file_name());

    // for file in dir_files {
    //     println!("{:?}",file);
    // }

    if dir_files.any(|file| file==start) {
        fs::rename(start, end)
            .expect("could not rename file");
        println!("moved {start} to {end}")
    } else {
        println!("file1.txt could not be found")
    }
}

