use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sha2::{Sha256, Digest};
use std::{
    fs,
    io::{self, Write},
    env,
};

const HEADER: &[u8] = b"ANH2";

fn key_paths() -> (String, String) {
    let home = env::var("HOME").expect("HOME not set");
    let dir = format!("{}/annihilator", home);
    let file = format!("{}/key.annihilator", dir);
    (dir, file)
}

fn read_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn destroy() {
    println!(r#"
    WARNING: FILE ANNIHILATION MODE

    This will irreversibly corrupt the file
    unless the key file is preserved.

    Type Y to continue:
    "#);

    if read_line().to_uppercase() != "Y" {
        println!("Aborted.");
        return;
    }

    print!("Input file path: ");
    io::stdout().flush().unwrap();
    let input = read_line();

    print!("Output file path: ");
    io::stdout().flush().unwrap();
    let output = read_line();

    let data = fs::read(&input).expect("Failed to read input");

    let salt: [u8; 32] = rand::thread_rng().gen();
    let mut hasher = Sha256::new();
    hasher.update(&salt);
    let seed = hasher.finalize();
    let mut rng = StdRng::from_seed(seed.into());

    let mut out: Vec<u8> = Vec::new();
    let mut garbage_lens: Vec<u8> = Vec::new();

    out.extend_from_slice(HEADER);
    out.extend_from_slice(&salt);

    for &b in &data {
        out.push(b);

        let g_len = rng.gen_range(0..=16);
        garbage_lens.push(g_len as u8);

        for _ in 0..g_len {
            out.push(rng.gen());
        }
    }

    fs::write(&output, out).expect("Write failed");

    let key_body = garbage_lens
    .iter()
    .map(|n| n.to_string())
    .collect::<Vec<_>>()
    .join(" ");

    let (key_dir, key_file) = key_paths();
    fs::create_dir_all(&key_dir).unwrap();

    fs::write(
        &key_file,
        format!("ANHKEY2\n{}", key_body)
    ).unwrap();

    println!("Key saved to {}", key_file);
}

fn restore() {
    print!("Destroyed file path: ");
    io::stdout().flush().unwrap();
    let input = read_line();

    print!("Key file path: ");
    io::stdout().flush().unwrap();
    let key_path = read_line();

    print!("Output restored file: ");
    io::stdout().flush().unwrap();
    let output = read_line();

    let file = fs::read(&input).expect("Read failed");
    let key_text = fs::read_to_string(&key_path).expect("Key read failed");

    let mut lines = key_text.lines();
    let header = lines.next().unwrap();
    if header != "ANHKEY2" {
        panic!("Invalid key format");
    }

    let garbage_lens: Vec<usize> = lines
    .next()
    .unwrap_or("")
    .split_whitespace()
    .map(|s| s.parse().expect("Invalid key data"))
    .collect();

    let mut pos = HEADER.len() + 32;
    let mut restored: Vec<u8> = Vec::new();

    for g_len in garbage_lens {
        if pos >= file.len() {
            break;
        }

        restored.push(file[pos]);
        pos += 1 + g_len;
    }

    fs::write(&output, restored).expect("Write failed");
    println!("Restore completed.");
}

fn main() {
    println!("Annihilator v0.2.0");
    println!("1) Destroy file");
    println!("2) Restore file");
    print!("Choose mode [1/2]: ");
    io::stdout().flush().unwrap();

    match read_line().as_str() {
        "1" => destroy(),
        "2" => restore(),
        _ => println!("Invalid choice"),
    }
}
