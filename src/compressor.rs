use std::{collections::BTreeMap, env, fs::read_to_string, io::Error, path::Path};

#[derive(Debug)]
pub struct Compressor {
    filename: String,
    char_count: BTreeMap<char, u32>,
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

            return Result::Ok(Compressor {
                filename: argument,
                char_count: BTreeMap::new(),
            });
        }

        Result::Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "No arguments provided",
        ))
    }

    pub fn compress(&mut self) {
        let buffer = read_to_string(&self.filename);

        buffer.unwrap().chars().for_each(|c| {
            self.add_char(c);
        });

        ()
    }

    fn add_char(&mut self, c: char) {
        *self.char_count.entry(c).or_insert(0) += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_test_file() {
        let mut compressor = Compressor {
            filename: String::from("test.txt"),
            char_count: std::collections::BTreeMap::new(),
        };
        compressor.compress();

        assert_eq!(compressor.char_count.get(&'X'), Some(&333));
        assert_eq!(compressor.char_count.get(&'t'), Some(&223000));
    }
}
