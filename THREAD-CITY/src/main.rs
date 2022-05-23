
use libc::ucontext_t;
use libc::c_char;
pub mod rb_thread;
use rb_thread::rb_thread_handler;
use std::mem;


// Función de ejemplo
extern "C" fn f1() {
    println!("INICIO 1");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(0), rb_thread::child_match(1));}
    println!("FIN 1");
}
extern "C" fn f2() {
    println!("INICIO 2");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(1), rb_thread::child_match(0));}
    println!("FIN 2");
}

extern "C" fn f3() {
    println!("Hilo 3");
}
pub fn main() {
    unsafe {
        let mut HANDLER = rb_thread::init_handler(1,1);
        let mut new_thread:rb_thread::rb_thread_t = rb_thread::create_thread(f1, 1);
        let mut new_thread:rb_thread::rb_thread_t = rb_thread::create_thread(f2, 2);

        HANDLER.start_threads();
    }




}