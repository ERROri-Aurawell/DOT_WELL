use crate::v1::var_maker::*;

use crate::v2::parse_values::catch_real_values;

pub fn change_var_value(pool: &mut Vec<Variable>, position: usize, novo_valor: &str) {
    pool[position].valor = match pool[position].tipo {
        Types::Bool => {
            let response = catch_real_values(novo_valor, pool, 1);

            match response {
                Ok(r) => match r {
                    Values::Bool(r) => Values::Bool(r),
                    _ => {
                        println!("Retorno inválido para tipo Bool");
                        return;
                    }
                },
                Err(r) => {
                    println!("Erro ao avaliar expressão: {}", r);
                    return;
                }
            }
        }
        Types::U8 => {
            let response = catch_real_values(novo_valor, pool, 2);

            match response {
                Ok(r) => match r {
                    Values::U8(r) => Values::U8(r),
                    _ => {
                        println!("Retorno inválido para tipo U8");
                        return;
                    }
                },
                Err(r) => {
                    println!("Erro ao avaliar expressão: {}", r);
                    return;
                }
            }
        }

        Types::I8 => {
            let response = catch_real_values(novo_valor, pool, 3);

            match response {
                Ok(r) => match r {
                    Values::I8(r) => Values::I8(r),
                    _ => {
                        println!("Retorno inválido para tipo I8");
                        return;
                    }
                },
                Err(r) => {
                    println!("Erro ao avaliar expressão: {}", r);
                    return;
                }
            }
        }

        Types::U32 => {
            let response = catch_real_values(novo_valor, pool, 4);

            match response {
                Ok(r) => match r {
                    Values::U32(r) => Values::U32(r),
                    _ => {
                        println!("Retorno inválido para tipo U32");
                        return;
                    }
                },
                Err(r) => {
                    println!("Erro ao avaliar expressão: {}", r);
                    return;
                }
            }
        }

        Types::I32 => {
            let response = catch_real_values(novo_valor, pool, 5);

            match response {
                Ok(r) => match r {
                    Values::I32(r) => Values::I32(r),
                    _ => {
                        println!("Retorno inválido para tipo I32");
                        return;
                    }
                },
                Err(r) => {
                    println!("Erro ao avaliar expressão: {}", r);
                    return;
                }
            }
        }
        _ => {
            println!(
                "String não pode ser alterada com CHANGE. Use DROP e crie uma nova variável. Não estoure a memória!"
            );
            return;
        }
    };
}
