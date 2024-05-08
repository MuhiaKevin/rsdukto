use std::collections::HashMap;
use std::io;
use std::fs::{self, DirEntry};
use std::os::unix::fs::MetadataExt;
use std::path::Path;


pub fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = std::path::Path::new(&args[1]);

    // for entry in path.read_dir().expect("read_dir call failed") {
    //     if let Ok(entry) = entry {
    //         println!("{:?}", entry.path());
    //     }
    // }

    // let show_entry: &dyn Fn(&DirEntry) = &|entry: &DirEntry|  println!("{:?}", entry.path());
    //
    // visit_dirs(path, show_entry ).unwrap()

    create_folder_info(path);
}



fn create_folder_info(path: &Path) {
    // let total_num_of_files = vec![0u8; 8];
    // let total_size_of_folder = vec![0u8; 8];
    // let something = vec![0xffu8; 8];
    // let mut num_of_files = 0;
    // let mut folder_size = 0;

    let root_name = path.file_name().unwrap().
        to_str().unwrap()
        .as_bytes();

    println!("{root_name:?}");

    let mut total_size: u64 = 0;
    let mut num_of_files: u64 = 0;
    let mut files_and_their_packets: HashMap<String, Vec<u8>> = HashMap::new();
    
    let mut closure =  |entry: &DirEntry|  {
        let path = entry.path();

        if path.exists() && path.is_file() && !path.is_symlink() {
            // set total size of all files in the folder and how many they are
            let file_size = path.metadata().unwrap().size();
            total_size += file_size;
            num_of_files += 1;

            println!("Filename {path:?} has size: {file_size}");
        }
    };

    let show_entry: Box<&mut dyn FnMut(&DirEntry)> = Box::new(&mut closure);
    visit_dirs(path, *show_entry).unwrap();

    println!("{num_of_files} files with total size of : {total_size} bytes");
}


fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
