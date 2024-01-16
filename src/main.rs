use {
    std::{ fs, sync::mpsc, process::Command, env, io::BufReader, io::BufRead, path::Path},
    tray_item::TrayItem,
    tray_item::IconSource
};

enum Modes {
    Quit,
    Perf,
    Pwr
}

fn main() {
    let mut bin_location = env::current_exe().expect("Could not get current exe location");
    bin_location.pop();
    let bin_dir = bin_location.to_str().expect("Could not format path into string");
    println!("Running in {}", bin_dir);

    let mut init_icon = IconSource::Resource("/home/junm/Trunk/Coding/Rust/tlp-switcher/src/sprites/default.png");
    
    if Path::new("/etc/tlp.d/tlp-switcher.conf").exists() {
        let current_file = fs::File::open("/etc/tlp.d/tlp-switcher.conf").expect("Failed to read tlp-switcher.conf");
        let mut buffer = BufReader::new(current_file);
        let mut first_line = String::new();
        let _ = buffer.read_line(&mut first_line);
        if first_line.ends_with('\n') { first_line.pop(); }
        println!("{first_line}");

        match first_line.as_str() {
            "TLP-switcher performance config" => { init_icon = IconSource::Resource("/home/junm/Trunk/Coding/Rust/tlp-switcher/src/sprites/performance.png"); },
            "TLP-switcher powersave config" => { init_icon = IconSource::Resource("/home/junm/Trunk/Coding/Rust/tlp-switcher/src/sprites/powersave.png"); }
            _ => {}
        };
    }
    
    let mut tray = TrayItem::new(
        "test tray icon",
        init_icon
    ).expect("Icon not found");

    let (tx,rx) = mpsc::sync_channel::<Modes>(2);

    let perf_tx = tx.clone();
    tray.add_menu_item(
        "Performance mode",
        move || {perf_tx.send(Modes::Perf).unwrap();} 
    ).unwrap();
    
    let pwr_tx = tx.clone();
    tray.add_menu_item(
        "Powersave mode",
        move || {pwr_tx.send(Modes::Pwr).unwrap();} 
    ).unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item(
        "Quit",
        move || {quit_tx.send(Modes::Quit).unwrap();} 
    ).unwrap();
    
    loop {
        match rx.recv() {
            Ok(Modes::Quit) => {
                println!("Quit");
                fs::remove_file("/etc/tlp.d/tlp-switcher.conf").expect("Failed to delete config file at /etc/tlp.d/tlp-switcher.conf");
                break;
            },
            Ok(Modes::Perf) => {
                println!("Performance Mode");
                fs::copy("configs/performance.conf", "/etc/tlp.d/tlp-switcher.conf").expect("Could not copy config to /etc/tlp.d/tlp-switcher.conf");
                tray.set_icon(
                    IconSource::Resource("/home/junm/Trunk/Coding/Rust/tlp-switcher/src/sprites/performance.png")
                ).unwrap();
                Command::new("sudo").args(["tlp", "start"]).output().expect("Failed to execute \"tlp start\"");
            },
            Ok(Modes::Pwr) => {
                println!("Powersave Mode");
                fs::copy("configs/powersave.conf", "/etc/tlp.d/tlp-switcher.conf").expect("Could not copy config to /etc/tlp.d/tlp-switcher.conf");
                tray.set_icon(
                    IconSource::Resource("/home/junm/Trunk/Coding/Rust/tlp-switcher/src/sprites/powersave.png")
                ).unwrap();
                Command::new("sudo").args(["tlp", "start"]).output().expect("Failed to execute \"tlp start\"");
            }
            _ => {}
        };
    };
}
