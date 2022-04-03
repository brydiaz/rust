use std::env;
use hstrace::prelude::*;

fn main() {
    
    let args: Vec<String> = env::args().collect(); //SE obtienen los parametro
    
    //Acá se salvan los que son solo parametros de prog
    let mut arguments_for_one = Vec::<String>::new();
    let mut control_one = 0;
    let mut control_two = 0;

    let mut arguments_for_two = Vec::<String>::new();

    for elm in args{
        if control_one > 3{
            arguments_for_one.push(elm);
        }
        control_one = control_one +1
    }
    let args: Vec<String> = env::args().collect(); //SE obtienen los parametro

    for elm in args{
        if control_two > 3{
            arguments_for_one.push(elm);
        }
        control_two = control_two +1
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
                read_syscalls_with_table(&program_name, arguments_for_one);//Opcion que muestra los syscalls de golpe
            }
            if option == "-V" {
                read_syscalls_with_control(&program_name, arguments_for_two)
            }
        } else {
            println!("Error en las opcioones");
        }
       
        
    } else {
        println!("Programa no inicializado");
    }
}

//Lee los syscalls, recibe el nombre del programa y los argumentos, imprime los syscalls utilizados
fn read_syscalls_with_table(program:&str, arguments:Vec<String>){
    let mut tracer;
    if arguments.len() == 0{
        tracer = HStraceBuilder::new().program(program).build();//Si no tiene argumentos no los configura
    }else {
        tracer = HStraceBuilder::new().program(program).args(arguments).build();
    }

    tracer.start().unwrap();//Inicia el tracer
    let mut syscalls = Vec::<String>::new();
    let mut syscalls_list = Vec::<(String, i32)>::new();
    for syscall in tracer.iter_as_syscall() {
        //println!("File operation detected: {:?}", syscall);
        if is_in(syscall.name.to_string(), &syscalls) == false{
            syscalls_list.push((syscall.name.to_string(),1));
            syscalls.push(syscall.name.to_string());
        } else {
            act_index(syscall.name.to_string(), &mut syscalls_list)   
        }
      
    }

    println!("{:?}","");
    print_table_array(&mut syscalls_list);
    println!("{:?}","");

}

//Verfica si el elemento está en el vector
fn is_in(element:String,vector:&Vec<String>)->bool{
    for i in vector{
        if i.to_string() == element{
            return true;
        }
    }
    return false;
}

//Actualiza en uno el elemento que haya en el vector
fn act_index(element:String,vector: &mut Vec<(String, i32)>){
    for  i in vector.iter_mut(){
        if i.0.to_string() == element{
            i.1 += 1
        }
    }

}

//Imprime bonito un arreglo de tuplas
fn print_table_array(vector: &mut Vec<(String, i32)>) {

    for i in vector{
        println!("SYSCALL: {:?} NUMBER OF TIMES: {}", i.0, i.1)
    }
}

//Lee los syscalls, recibe el nombre del programa y los argumentos, imprime los syscalls utilizados con una tecla controlada
fn read_syscalls_with_control(program:&str, arguments:Vec<String>){
    let mut tracer;
    if arguments.len() == 0{
        tracer = HStraceBuilder::new().program(program).build();//Si no tiene argumentos no los configura
    }else {
        tracer = HStraceBuilder::new().program(program).args(arguments).build();
    }

    tracer.start().unwrap();//Inicia el tracer
    let mut syscalls = Vec::<String>::new();
   
    for syscall in tracer.iter_as_syscall() {
        println!("{:?}",syscall);
        let mut line = String::new();
        println!("Press B to continue!");
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
    }


}