use std::{fs::File, io::Write, process::Command, time::Instant};

fn main() {
    for i in 0..8 {
        let path = format!("time{}.txt", i + 1);
        let mut file = File::create(path).unwrap();

        let _ = file.write(format!("File {}\n", i).as_bytes());

        for x in 0..10 {
            let now = Instant::now();
            let _ = Command::new("sh")
                .args(["-c", format!("cp {}.txt ~/", i).as_str()])
                .output()
                .expect("failed to execute process");

            let elapsed = now.elapsed();

            let _ = file.write(format!("\tElapsed {} ms\n", elapsed.as_millis()).as_bytes());

            let _ = Command::new("sh")
                .args(["-c", format!("rm ~/{}.txt", i).as_str()])
                .output()
                .expect("failed to execute process");
        }
    }
}
