use std::ptr;

pub enum Types {
    Bool,
    U8,
    I8,
    U32,
    I32,
    String,
}

pub enum Values {
    Bool(bool),
    U8(u8),
    I8(i8),
    U32(u32),
    I32(i32),
    String(String),
}

pub enum Pointers {
    Bool(*mut bool),
    U8(*mut u8),
    I8(*mut i8),
    U32(*mut u32),
    I32(*mut i32),
    String(*mut String),
}
pub struct Variable {
    pub valor: Values,
    pub tipo: Types,
    pub ponteiro: Pointers,
    pub nome: String,
}

pub struct Escopo {
    pub nome: String,
    pub codigo: String,
}

impl Variable {
    pub unsafe fn destruidor(&self) {
        match self.ponteiro {
            Pointers::Bool(ptr) => ptr::drop_in_place(ptr),
            Pointers::U8(ptr) => ptr::drop_in_place(ptr),
            Pointers::I8(ptr) => ptr::drop_in_place(ptr),
            Pointers::U32(ptr) => ptr::drop_in_place(ptr),
            Pointers::I32(ptr) => ptr::drop_in_place(ptr),
            Pointers::String(ptr) => ptr::drop_in_place(ptr),
        }
    }
}

pub unsafe fn create_var(
    tipo: Types,
    valor: Values,
    nome: &str,
    pool: &mut Vec<Variable>,
) -> *const Variable {
    let ponteiro = match &valor {
        Values::Bool(v) => Pointers::Bool(Box::into_raw(Box::new(*v))),
        Values::U8(v) => Pointers::U8(Box::into_raw(Box::new(*v))),
        Values::I8(v) => Pointers::I8(Box::into_raw(Box::new(*v))),
        Values::U32(v) => Pointers::U32(Box::into_raw(Box::new(*v))),
        Values::I32(v) => Pointers::I32(Box::into_raw(Box::new(*v))),
        Values::String(v) => Pointers::String(Box::into_raw(Box::new(v.clone()))),
    };

    let var = Variable {
        valor,
        tipo,
        ponteiro,
        nome: nome.to_string(),
    };

    pool.push(var);

    pool.last().unwrap() as *const Variable
}

pub unsafe fn remover_var(pool: &mut Vec<Variable>, var_ptr: *const Variable) {
    if let Some(pos) = pool.iter().position(|v| v as *const Variable == var_ptr) {
        match pool[pos].ponteiro {
            Pointers::Bool(ptr) => {
                let _ = Box::from_raw(ptr);
            }
            Pointers::U8(ptr) => {
                let _ = Box::from_raw(ptr);
            }
            Pointers::I8(ptr) => {
                let _ = Box::from_raw(ptr);
            }
            Pointers::U32(ptr) => {
                let _ = Box::from_raw(ptr);
            }
            Pointers::I32(ptr) => {
                let _ = Box::from_raw(ptr);
            }
            Pointers::String(ptr) => {
                let _ = Box::from_raw(ptr);
            }
        }

        pool.remove(pos);
    }
}

pub unsafe fn read_variables(pool: &mut Vec<Variable>) {
    println!("\n---- Variáveis alocadas ----");

    for var in pool {
        print!("{}: ", var.nome);

        match var.tipo {
            Types::Bool => {
                if let Values::Bool(v) = var.valor {
                    println!("Bool = {}", v);
                }
            }
            Types::U8 => {
                if let Values::U8(v) = var.valor {
                    println!("U8 = {}", v);
                }
            }
            Types::I8 => {
                if let Values::I8(v) = var.valor {
                    println!("I8 = {}", v);
                }
            }
            Types::U32 => {
                if let Values::U32(v) = var.valor {
                    println!("U32 = {}", v);
                }
            }
            Types::I32 => {
                if let Values::I32(v) = var.valor {
                    println!("I32 = {}", v);
                }
            }
            Types::String => {
                if let Values::String(ref v) = var.valor {
                    println!("String = \"{}\"", v);
                }
            }
        }
    }
}

pub fn find_var_value(name: &str, pool: &Vec<Variable>) -> Result<String, String> {
    for var in pool {
        if var.nome == name {
            return Ok(match &var.valor {
                Values::Bool(v) => (if *v { 1 } else { 0 }).to_string(),
                Values::U8(v) => v.to_string(),
                Values::I8(v) => v.to_string(),
                Values::U32(v) => v.to_string(),
                Values::I32(v) => v.to_string(),
                Values::String(v) => v.clone(),
            });
        }
    }
    Err(format!("Variável '{}' não encontrada.", name))
}
