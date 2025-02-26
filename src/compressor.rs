mod huffman;

use huffman::HuffTree;
use std::{collections::BTreeMap, env, fs::read_to_string, io::Error, path::Path};

#[derive(Debug)]
pub struct Compressor {
    filename: String,
}

impl Compressor {
    pub fn init() -> Result<Compressor, Error> {
        for argument in env::args().skip(1) {
            let path = Path::new(&argument);
            if !path.exists() {
                return Result::Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("File not found: {}", argument),
                ));
            }

            return Result::Ok(Compressor { filename: argument });
        }

        Result::Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "No arguments provided",
        ))
    }

    pub fn compress(&mut self) {
        let tree = self.create_tree();
        let map = tree.generate_prefix_map();
        dbg!(map);
        ()
    }

    fn create_tree(&self) -> HuffTree {
        let buffer = read_to_string(&self.filename);
        let mut map = BTreeMap::new();
        buffer.unwrap().chars().for_each(|c| {
            *map.entry(c).or_insert(0) += 1;
        });

        HuffTree::from(map)
    }
}
