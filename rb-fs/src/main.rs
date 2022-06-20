mod mkfs;
mod ses_infor;
use std::env;
use std::ffi::OsStr;
use image;
use quircs;


fn main() {

    let mountpoint = env::args().nth(2).unwrap();
    let disk_direction = env::args().nth(1).unwrap();
    let fs = mkfs::Rb_fs::new(mountpoint.clone(), disk_direction.clone());
    let options = ["-o", "nonempty"].iter().map(|o| o.as_ref()).collect::<Vec<&OsStr>>();
    println!("RB-FS started!");
    fuse::mount(fs, &mountpoint, &options).unwrap();
    
}

// Prepare for detection




