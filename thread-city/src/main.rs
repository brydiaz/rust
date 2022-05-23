mod mypthreads;
use std::time;
use std::thread;
use std::process;

//arguments:Vec<mypthreads::Types>
fn thread_test() -> isize{
    println!("Thread: {}", process::id());

    while true {
        println!("1");
    }
    0
}
fn thread_test2() -> isize{
    println!("Thread: {}", process::id());

    while true {
        println!("2");
    }
    0
}


fn main() {
    mypthreads::my_thread_create(&thread_test);
    mypthreads::my_thread_create(&thread_test2);
}
