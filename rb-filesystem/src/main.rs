extern crate env_logger;
extern crate fuse;
extern crate libc;
extern crate time;

use std::env;
use std::ffi::OsStr;
use libc::ENOENT;
use time::Timespec;
use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };                     // 1 second

const CREATE_TIME: Timespec = Timespec { sec: 1381237736, nsec: 0 };    // 2013-10-08 08:56 Date time

const DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: CREATE_TIME,
    mtime: CREATE_TIME,
    ctime: CREATE_TIME,
    crtime: CREATE_TIME,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

const FILE_ATTR: FileAttr = FileAttr {
    ino: 2,
    size: 13,
    blocks: 1,
    atime: CREATE_TIME,
    mtime: CREATE_TIME,
    ctime: CREATE_TIME,
    crtime: CREATE_TIME,
    kind: FileType::RegularFile,
    perm: 0o644,
    nlink: 1,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

//Here we save the directories names
static mut DIR_LIST:Vec<String> = Vec::new();
//Here we save the files names
static mut FILE_LIST:Vec<String> = Vec::new();
//Here we save the contents of the files
static mut FILE_CONTENT:Vec<String> = Vec::new();

//We add one directory
fn add_dir(dir_name:&String) {
    unsafe {DIR_LIST.push(dir_name.to_string());}
}


//We check if the path is in the directory paths
fn is_dir(path:&String) -> i32 {
    unsafe{
        let new_path = cut_first_letter(&path);
        for i in &DIR_LIST{
            if &new_path == i{
                return 1
            }
        }
    }
    return 0;
}

//We add one file with content in blank
fn add_file(filename:&String) {
    unsafe {
        FILE_LIST.push(filename.to_string());
        FILE_CONTENT.push("".to_string());
    }
}

//We check if the file is in the list of files
fn is_file(path:&String) -> i32 {
    unsafe{
        let new_path = cut_first_letter(&path);
        for i in &FILE_LIST{
            if &new_path == i{
                return 1
            }
        }
    }
    return 0;
}

//Given one path search for the index if exists the file in the path
fn get_file_index(path:&String) -> i32{
    unsafe{
        let mut j:i32 = 0;
        let new_path = cut_first_letter(&path);
        for i in &FILE_LIST{
            if &new_path == i{
                return j;
            }
            j+=1;
        }
    }
    return -1;
}

//Prints an array of strings
fn print_array(array:&Vec<String>) {
    for i in array{
        println!("{}",i);
    }
}

//Cut the first letter of the string and return it
fn cut_first_letter(string:&String) -> String {
    let mut string_end = "".to_string();
    let mut index = 0;
    for c in string.chars() {
        if index == 0{
            index += 1;
        } else {
            string_end = string_end + &c.to_string();
        }
    }
    return string_end.to_string();
}

//Change the content of the file in the given path
fn write_to_file(path:&String, content:&String) -> i32{
    let index:usize = get_file_index(path).try_into().unwrap();
    if index < 0 {
        return -1
    } else {
        unsafe{FILE_CONTENT[index] = content.to_string();
        return 0;
        }
    }
}


struct HelloFS;

impl Filesystem for HelloFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 && name.to_str() == Some("hello.txt") {
            reply.entry(&TTL, &FILE_ATTR, 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        match ino {
            1 => reply.attr(&TTL, &DIR_ATTR),
            2 => reply.attr(&TTL, &FILE_ATTR),
            _ => reply.error(ENOENT),
        }
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, _size: u32, reply: ReplyData) {
        if ino == 2 {
            reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        let entries = vec![
            (1, FileType::Directory, "."),
            (1, FileType::Directory, ".."),
        ];

        for i in DIR_LIST {
            entries.push((2, FileType::RegularFile, &i));
        }
        // Offset of 0 means no offset.
        // Non-zero offset means the passed offset has already been seen, and we should start after
        // it.
        let to_skip = if offset == 0 { offset } else { offset + 1 } as usize;
        for (i, entry) in entries.into_iter().enumerate().skip(to_skip) {
            reply.add(entry.0, i as i64, entry.1, entry.2);
        }
        reply.ok();
    }
}

fn main() {
    let mut path = "dev/test/".to_string();
    let mut file = "hola_mundo.txt".to_string();
    let mut file_path = "/hola_mundo.txt".to_string();
    add_dir(&path);
    add_file(&file);
    unsafe{
        print_array(&DIR_LIST);
        print_array(&FILE_LIST);
        print_array(&FILE_CONTENT);
        let content =  "Hey esto es un hola mundo".to_string();
        write_to_file(&file_path, &content);
        print_array(&FILE_CONTENT);
    }
}