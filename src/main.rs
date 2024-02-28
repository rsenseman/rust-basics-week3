use std::env;

#[derive(Debug)]
#[allow(dead_code)]
struct FileSize {
    bytes: u64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
}

impl FileSize {
    fn new_from_bytes(size_bytes: i32) -> FileSize {
        let new_filesize: FileSize = FileSize {
            bytes: size_bytes as u64,
            kilobytes: size_bytes as f64 / 1000.0,
            megabytes: size_bytes as f64 / 1000000.0,
            gigabytes: size_bytes as f64 / 1000000000.0,
        };
        new_filesize
    }

    fn new(int_spec: i32, magnitude_spec: &str) -> FileSize {
        let power: u32 = match magnitude_spec {
            "BYTES" => 0,
            "KB" => 3,
            "MB" => 6,
            "GB" => 9,
            _ => panic!("Magnitude spec not recognized: {}", magnitude_spec),
        };

        let base: i32 = 10;
        let new_filesize = FileSize::new_from_bytes(int_spec * base.pow(power));

        new_filesize
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let spec = args[1].clone();
    let spec_vec: Vec<String> = spec
        .trim()
        .split(" ")
        .map(|val| val.parse().unwrap())
        .collect();
    let (num, magnitude) = (spec_vec[0].parse::<i32>().unwrap(), spec_vec[1].to_uppercase());

    // The first argument is the size that was used to call the program. Must use quotes to
    // read this as a single argument
    println!("Input is:\nSize Int: {}\nMagnitude: {}", num, magnitude);

    let result: FileSize = FileSize::new(num, magnitude.as_str());
    println!("Sizes: {:?}", result);
}