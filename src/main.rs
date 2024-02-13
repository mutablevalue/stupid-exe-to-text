use std::{ env, io::Read, io::self, process, string, fs::File, io::Write, fmt };

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path>", args[0]);
        process::exit(1);
    }
    let exe_path = &args[0]; 
    let output_path = &args[1];
    let mut file = File::open(exe_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let binary_string = String::from_utf8_lossy(&buffer);
    
    let mut formatted_string = String::new();
    for (i, c) in binary_string.chars().enumerate() {
        formatted_string.push(c);
        if (i + 1) % 100  == 0 {
            formatted_string.push('\n');
        }
    }

    let mut output_file = File::create(output_path)?;
    output_file.write_all(formatted_string.as_bytes())?;

    println!("Data written to {:?}", output_file);

    Ok(())
}