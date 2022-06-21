mod mkfs;
mod ses_infor;
mod fsck;
mod mount;
use std::env;
use std::ffi::OsStr;
use image;
use quircs;


fn main() {
    mount::mount();
}





