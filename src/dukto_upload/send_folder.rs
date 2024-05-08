use std::io;
use std::fs::{self, DirEntry};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::collections::HashMap;

pub fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = std::path::Path::new(&args[1]);
    // for entry in path.read_dir().expect("read_dir call failed") {
    //     if let Ok(entry) = entry {
    //         println!("{:?}", entry.path());
    //     }
    // }

    create_folder_info(path);
    // let show_entry: &dyn Fn(&DirEntry) = &|entry: &DirEntry|  println!("{:?}", entry.path());
    //
    // visit_dirs(path, show_entry ).unwrap()
}



fn create_folder_info(path: &Path) {
    let root_name = path.file_name().unwrap().
        to_str().unwrap()
        .as_bytes();

    let total_num_of_files = vec![0u8; 8];
    let total_size_of_folder = vec![0u8; 8];
    //    let something = vec![0xffu8; 8];
    //
    //    let mut num_of_files = 0;
    //    let mut folder_size = 0;
    //    let mut files_and_their_packets: HashMap<String, Vec<u8>> = HashMap::new();


    let show_entry: &dyn Fn(&DirEntry) = &|entry: &DirEntry|  {
        let path = entry.path();

        if path.exists() && path.is_file() && !path.is_symlink() {
            let file_size = path.metadata().unwrap().size();

            println!("Filename {path:?} has size: {file_size}");
        }
    };

    visit_dirs(path, show_entry ).unwrap();

    // totalSizeBytes := make([]byte, 8)
    // afterFileNameBytes := make([]byte, 1)
    // endingBytes := bytes.Repeat([]byte{255}, 8)

    println!("{root_name:?}");
}


// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
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









// // let path = Path::new("/home/muhia/Downloads/Your Honor/");
// let path = Path::new("../../../../Downloads/Your Honor/");
//
// if path.is_dir() && !path.is_symlink() {
//     println!("{}", path.is_dir());
// }
//
//
//
// println!("{}", path.is_absolute());
// println!("{}", path.is_relative());
//
//
// let path = std::path::Path::new("/tmp");
// for entry in path.read_dir().expect("read_dir call failed") {
//     if let Ok(entry) = entry {
//         println!("{:?}", entry.path());
//     }
// }

//
// fn main() {
//     let args: Vec<_> = std::env::args().collect();
//     let path = std::path::Path::new(&args[1]);
//     for entry in path.read_dir().expect("read_dir call failed") {
//         if let Ok(entry) = entry {
//             println!("{:?}", entry.path());
//         }
//     }
//
//     let show_entry: &dyn Fn(&DirEntry) = &|entry: &DirEntry|  println!("{:?}", entry.path());
//
//     visit_dirs(path, show_entry ).unwrap()
// }
//






