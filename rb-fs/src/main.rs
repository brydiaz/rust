extern crate env_logger;
extern crate fuse;
extern crate libc;
extern crate time;

use std::env;
use std::ffi::OsStr;
use libc::ENOENT;
use time::Timespec;
use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};


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
    let index:usize = get_file_index(path).try_into().unwrap();;
    if index < 0 {
        return -1
    } else {
        unsafe{FILE_CONTENT[index] = content.to_string();
        return 0;
        }
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