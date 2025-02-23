pub mod compressor;

fn main() {
    match compressor::Compressor::init() {
        Ok(mut compressor) => {
            compressor.compress();
            dbg!(compressor);
        }
        Err(error) => {
            println!("Error initializing compressor: {}", error);
        }
    }
}
