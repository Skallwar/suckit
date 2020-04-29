use std::fs;
use std::io::Write;
use std::path::PathBuf;

//TODO: Recover insted of panic
pub fn save_file(file_name: &str, content: &[u8], path: &Option<PathBuf>) {
    let path = match path {
        Some(path) => path.join(file_name),
        None => PathBuf::from(file_name),
    };

    if let Some(parent) = path.parent() {
        match fs::create_dir_all(parent) {
            Err(err) => panic!("Couldn't create folder {}: {}", parent.display(), err),
            Ok(()) => (),
        }
    }

    let mut file = match fs::File::create(&path) {
        Err(err) => panic!("Couldn't create {}: {}", path.display(), err),
        Ok(file) => file,
    };

    if let Err(err) = file.write_all(content) {
        panic!("Couldn't write to {}: {}", path.display(), err);
    }
}

pub fn symlink(source: &str, destination: &str, path: &Option<PathBuf>) {
    let destination = match path {
        Some(path) => path.join(destination),
        None => PathBuf::from(destination),
    };

    std::os::unix::fs::symlink(source, destination).unwrap();
}
