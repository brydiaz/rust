use crate::mkfs;
use crate::fsck;
use crate::ses_infor;
use std::env;
use std::ffi::OsStr;
use image;
use quircs;


pub fn mount() {

    let mountpoint = env::args().nth(2).unwrap();
    let disk_direction = env::args().nth(1).unwrap();
    let fs = mkfs::Rb_fs::new(mountpoint.clone(), disk_direction.clone());
    fsck::check_consistens(&fs);
    let options = ["-o", "nonempty"].iter().map(|o| o.as_ref()).collect::<Vec<&OsStr>>();
    println!("RB-FS started!");
    fuse::mount(fs, &mountpoint, &options).unwrap();
    
}
