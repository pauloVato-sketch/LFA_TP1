use std::path::Path;

fn main() {
    println!("Y or N {}", Path::new("testFiles/mt.json").is_file());
}
