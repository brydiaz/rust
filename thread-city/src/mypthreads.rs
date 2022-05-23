extern crate nix;
use std::process;
use nix::sched::{self, CloneFlags};

pub fn my_thread_create(func: &dyn  Fn()->isize) {
    const STACK_SIZE: usize = 1024 * 1024;
    let ref mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let cbk = Box::new(|| func());
    let pid = sched::clone(cbk, stack, CloneFlags::CLONE_FS | CloneFlags::CLONE_FILES | CloneFlags::CLONE_SIGHAND | CloneFlags::CLONE_VM | CloneFlags::CLONE_THREAD | CloneFlags::CLONE_VFORK  , Some(signal_hook::consts::SIGCHLD as i32));
}