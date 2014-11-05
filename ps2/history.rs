use std::io::fs::PathExtensions;
use std::io::{BufferedReader, File, Append, ReadWrite};
use std::io::{SeekSet, SeekEnd};


pub struct History {
    file : File,
    index: u64,
}

impl History {

    pub fn new(filepath: &str) -> History {
        let mut index: u64 = 1;
        let path = Path::new(filepath);

        if path.exists() {
            let mut file = BufferedReader::new(File::open(&path));

            for _ in file.lines() {
                index += 1;
            }
        }

        let mut file = File::open_mode(&path, Append, ReadWrite).unwrap();
        file.seek(0, SeekEnd);
        History {file: file, index: index}
    }

    pub fn read(&mut self) {
        self.file.seek(0, SeekSet);
        print!("{}", String::from_utf8(self.file.read_to_end().unwrap()).unwrap());
        self.file.seek(0, SeekEnd);
    }

    pub fn write(&mut self, cmd: &str) {
        if cmd != "history" {
            self.file.write_str(format!("{} {}\n", self.index, cmd).as_slice());
            self.index += 1;
        }
    }
}
