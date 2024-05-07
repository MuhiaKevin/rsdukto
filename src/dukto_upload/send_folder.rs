

// let path = Path::new("/home/muhia/Downloads/Your Honor/");
let path = Path::new("../../../../Downloads/Your Honor/");

if path.is_dir() && !path.is_symlink() {
    println!("{}", path.is_dir());
}



println!("{}", path.is_absolute());
println!("{}", path.is_relative());
