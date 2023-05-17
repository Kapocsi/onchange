use std::env::args;
use std::fs::read;
use std::time::Duration;

fn main() -> Result<(), ()> {
    let mut old_file: Vec<u8> = Vec::new();
    let mut first_run = true;
    dbg!(&old_file);
    loop {
        let args: Vec<String> = args().collect();

        if let Some(file_path) = args.get(1) {
            if let Ok(file) = read(file_path) {
                if old_file != file && !first_run {
                    break;
                }
                first_run = false;
                old_file = file;
            } else {
                println!("Err Failed to read file");
                break;
            }
        } else {
            println!("Err No file providied");
            break;
        }

        std::thread::sleep(Duration::from_secs(1))
    }

    Ok(())
}
