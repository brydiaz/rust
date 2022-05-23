extern crate libc;

use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void};
use std::mem;

//Contextos basicos, donde estamos y donde vamos
static mut PARENT: Option<ucontext_t> = None;
static mut THREADS:Vec<rb_thread_t> = Vec::new();
static mut THREADS_CONTEXT:Vec<Option<ucontext_t>> = Vec::new();
static mut ID_THREAD:isize = 0;


//Estructura del hilo
#[derive(Copy, Clone)]
pub struct rb_thread_t {
    id: isize,
    priority: isize,
    context: ucontext_t
}

pub struct rb_thread_handler {
    pub sche_type: isize,
    pub quatum_time:u32
}impl rb_thread_handler {
    pub fn start_threads(&mut self) {
        // "self" refers to the value this method is being called on
        if self.sche_type == 1 {
            println!("RoundRobin");
            self.sch_round_robin();
        } else if self.sche_type == 2 {
            println!("Priority time");
        }
    }
    pub fn sch_round_robin(&mut self) {
        unsafe{
            let mut i:usize =0;
            while i != THREADS_CONTEXT.len(){
                thread_yield(parent_match() as *mut ucontext_t, child_match(i) as *const ucontext_t );
                i+=1;
            }
        }
   
    }



    pub fn thread_chsched(&mut self, sche:isize){
        self.sche_type = sche;
    }
}




pub fn create_thread(func: extern "C" fn(), priority_thread: isize) -> rb_thread_t{
    
    unsafe {
        let mut st1: [c_char; 8192] = [mem::zeroed(); 8192];
        
        let mut child_temp: ucontext_t = mem::uninitialized();
        getcontext(&mut child_temp as *mut ucontext_t);
        child_temp.uc_stack.ss_sp = st1.as_mut_ptr() as *mut c_void;
        child_temp.uc_stack.ss_size = mem::size_of_val(&st1);

        child_temp.uc_link = parent_match() as *mut ucontext_t;
       
        makecontext(&mut child_temp as *mut ucontext_t, func, 0);
        let thread_t = rb_thread_t {id:ID_THREAD, priority:priority_thread, context:  child_temp };
        //Thread creado
        THREADS.push(thread_t);
        THREADS_CONTEXT.push(Some(child_temp));
        let thread_t = rb_thread_t {id:ID_THREAD, priority:2, context: child_temp };
        ID_THREAD += 1;
        return thread_t;
    }

    
}

pub fn thread_end(context: *mut ucontext_t) {
    unsafe{swapcontext(context, parent_match() as *mut ucontext_t)};
}
  
pub fn thread_yield(context_from: *mut ucontext_t, context_to: *const ucontext_t) {
    unsafe{swapcontext(context_from as *mut ucontext_t, context_to as *const ucontext_t)};
}


// Funcion para acceder al padre
unsafe fn parent_match() -> &'static mut ucontext_t {
    match PARENT {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

pub unsafe fn child_match(i:usize) -> &'static mut ucontext_t {
    match THREADS_CONTEXT[i] {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}
pub fn init_handler(s:isize, q:u32) -> rb_thread_handler{
    unsafe{PARENT = Some(mem::uninitialized());}
    let mut handler = rb_thread_handler {sche_type:s, quatum_time:q};
    return handler;
}

