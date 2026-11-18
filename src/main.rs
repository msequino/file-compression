fn main() {
    menu();
//    compress_file(input_file, output_file, algorithm);
}

fn menu() {
    println!("Welcome to file-compression in Rust!");
    println!("This is a simple file compression tool written in Rust.");
    println!("--------------------------------------");
    println!("Usage: file-compression <input_file> <output_file>");
    println!("--algorithm <algorithm> : Specify the compression algorithm (default: gzip)");
}
fn compress_file(input_file: &str, output_file: &str, algorithm: &str) {
    // Placeholder for the actual compression logic
    println!("Compressing '{}' to '{}' using '{}' algorithm...", input_file, output_file, algorithm);
    // Here you would implement the actual compression logic based on the specified algorithm
}
