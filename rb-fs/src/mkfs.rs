use fuse::{FileType};
use fuse::{FileAttr};

//LLevaremos un control de los inodes 
static mut NEXT_INO: u64 = 1;

//Los Inodes son la unidad que movera nuestro fs
pub struct Inode {
    pub name: String,
    pub attributes : FileAttr,
    pub references: Vec<usize>
}

impl Inode {
    //Agrega una referencia a si mismo
    pub fn add_reference(&mut self,ref_value: usize) {
        self.references.push(ref_value);
    }
    //Elimina una referencia a si mismo
    pub fn delete_reference(&mut self,ref_value: usize) {
        self.references.retain(|i| *i != ref_value);
    }
}

//Creamos una estructura para guardar nuestros archivos Inodes
pub struct Disk {
    super_block : Vec<Inode>,
    pub root_path: String
}
impl Disk {
    //Crea un nuevo disco y crea el inode raiz
    pub fn new(path:String) -> Disk{
        unsafe{
            let mut blocks = Vec::new(); //Aca guardamos los inodes
            let ts = time::now().to_timespec();
            let attr = FileAttr {
                ino: NEXT_INO,
                size: 0,
                blocks: 0,
                atime: ts,
                mtime: ts,
                ctime: ts,
                crtime: ts,
                kind: FileType::Directory,
                perm: 0o755,
                nlink: 0,
                uid: 0,
                gid: 0,
                rdev: 0,
                flags: 0,
            };
            let name = ".";
            let initial_node = Inode {
                name : name.to_string(),
                attributes : attr,
                references : Vec::new()
            };
            
            blocks.push(initial_node);
            Disk {
                super_block : blocks,
                root_path :  path
            }
        }
        
    }
    //Retorna el siguiente ino disponible
    pub fn new_ino(&self) -> u64{
        unsafe{
            NEXT_INO = NEXT_INO +1;
            return NEXT_INO;
        }
        
    }
    //Agrega el inode al super bloque
    pub fn write_ino(&mut self, inode:Inode) {
        self.super_block.push(inode);
    }
    //Elimina el inode disponible
    pub fn remove_inode(&mut self, inode:Inode) {
        self.super_block.retain(|i| i.attributes.ino != inode.attributes.ino);
    }
    //Elimina una referencia de un respectivo inode
    pub fn clear_reference(&mut self, ino: u64, ref_value: usize) {
        for i in 0..self.super_block.len() {
            if self.super_block[i].attributes.ino == ino {
                self.super_block[i].delete_reference(ref_value);
            }
         }
    }
    //Agrega una respectiva referencia a un inode
    pub fn add_reference(&mut self, ino: u64, ref_value: usize) {
        for i in 0..self.super_block.len() {
            if self.super_block[i].attributes.ino == ino {
                self.super_block[i].add_reference(ref_value);
            }
         }
    }
     //Obtiene un Inode o nada
    pub fn get_inode(&self, ino: u64) -> Option<&Inode> {
        for i in 0..self.super_block.len() {
            if self.super_block[i].attributes.ino == ino {
                return Some(&self.super_block[i]);
            }

         }
         return None;
    }
    //Obtiene un Inode mutable o nada
    pub fn get_mut_inode(&mut self, ino: u64) -> Option<&Inode> {
        for i in 0..self.super_block.len() {
            if self.super_block[i].attributes.ino == ino {
                return Some(&mut self.super_block[i]);
            }

         }
         return None;
    }
    //Busca en base a la carpeta del padre el hijo que tenga el nombre por parametro
    pub fn find_inode_in_references_by_name(&self, parent_inode_ino: u64, name: &str) -> Option<&Inode> {
        for i in 0..self.super_block.len() {
           if self.super_block[i].attributes.ino == parent_inode_ino {
            let parent =  &self.super_block[i];
            for j in 0..parent.references.len() {
                for k in 0..self.super_block.len() {
                    if self.super_block[k].attributes.ino == parent.references[j].try_into().unwrap() {
                        let child =  &self.super_block[k];
                        if child.name == name {
                            return Some(child);
                        }
                    }
                }
            }
           }
        }
        
        return None;
        
    }
}

fn print_references(array:&Vec<usize>){
    for i in array {
        print!("{}",i);
    }
}

fn print_array(array:&Vec<Inode>){
    for i in array {
        print!("{}\n",i.attributes.ino);
    }
}
pub fn run() {
    let root_path = "./";
    let ts = time::now().to_timespec();
    let mut disk = Disk::new(root_path.to_string());
    
}