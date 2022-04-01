use std::env;
use hstrace::prelude::*;

fn main() {
    
    let args: Vec<String> = env::args().collect(); //SE obtienen los parametro
    
    //Acá se salvan los que son solo parametros de prog
    let mut arguments = Vec::<String>::new();
    let mut control = 0;

    for elm in args{
        if control > 3{
            arguments.push(elm)
        }
        control = control +1
    }

    let args: Vec<String> = env::args().collect();//Se recuperan los parametros

    let info = &args[0];//Info inicial del vector args
    let program = &args[1];//Accion basica del programa
    let option = &args[2];//Opcion que se busca ejecutar
    let program_name = &args[3];//nombre de prog

    //Flujo de control
    if program == "rastreador"{
        if option == "-v" || option == "-V"{
            if option == "-v"{
               read_syscalls(&program_name, arguments);//Opcion que muestra los syscalls de golpe
            }
            if option == "-V" {
                println!("opcion 2");
            }
        } else {
            println!("Error en las opcioones");
        }
       
        
    } else {
        println!("Programa no inicializado");
    }
}

//Lee los syscalls, recibe el nombre del programa y los argumentos, imprime los syscalls utilizados
fn read_syscalls(program:&str, arguments:Vec<String>){
    let mut tracer;
    if arguments.len() == 0{
        println!("tiene 0");
        tracer = HStraceBuilder::new().program(program).build();//Si no tiene argumentos no los configura
    }else {
        println!("no tiene 0");
        tracer = HStraceBuilder::new().program(program).args(arguments).build();
    }

    tracer.start().unwrap();//Inicia el tracer
    for syscall in tracer.iter_as_syscall() {
        match syscall.name {
            hstrace::Ident::Openat | hstrace::Ident::Fstat | hstrace::Ident::Stat => {
                println!("");
                println!("File operation detected: {:?}", syscall);
            }
            _ => (),
        }
    }
    
}