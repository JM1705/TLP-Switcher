use {
    std::{ fs, sync::mpsc, process::Command, env},
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

    let mut tray = TrayItem::new(
        "test tray icon",
        IconSource::Resource("~/Trunk/Coding/Rust/tlp-switcher/src/sprites/default.png")
    ).expect("Icon not found");

    let (tx,rx) = mpsc::sync_channel::<Modes>(2);

    let perf_tx = tx.clone();
    tray.add_menu_item(
        "Perfor but how to execute a root command and pass the password again to the command?mance mode",
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
                break;
            },
            Ok(Modes::Perf) => {
                println!("Performance Mode");
                tray.set_icon(
                    IconSource::Resource("~/Trunk/Coding/Rust/tlp-switcher/src/sprites/default.png")
                ).unwrap();
                Command::new("sudo").args(["tlp", "start"]).output().expect("Failed to execute \"tlp start\"");
            },
            Ok(Modes::Pwr) => {
                println!("Powersave Mode");
                tray.set_icon(
                    IconSource::Resource("~/Trunk/Coding/Rust/tlp-switcher/src/sprites/default.png")
                ).unwrap();
                Command::new("sudo").args(["tlp", "start"]).output().expect("Failed to execute \"tlp start\"");
            }
            _ => {}
        };
    };
}
