use rand::Rng;
use std::env;
use std::fs;
use std::io::{self, Write};
use tempfile::NamedTempFile;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} <number_of_rounds> <use_zeros: true|false>",
            args[0]
        );
        std::process::exit(1);
    }

    let num_rounds: u32 = args[1]
        .parse()
        .expect("Number of rounds must be an integer");
    let use_zeros: bool = args[2].parse().expect("use_zeros must be true or false");

    for _ in 0..num_rounds {
        let temp_file = NamedTempFile::new()?;

        let block_size = 4096;

        let mut buf = vec![0; block_size];
        if !use_zeros {
            let mut rng = rand::thread_rng();
            rng.fill(&mut buf[..]);
        }

        loop {
            match temp_file.as_file().write_all(&buf) {
                Ok(_) => continue,
                Err(err) if err.raw_os_error() == Some(libc::ENOSPC) => break,
                Err(err) => return Err(err),
            }
        }

        fs::remove_file(temp_file)?;
    }

    Ok(())
}
