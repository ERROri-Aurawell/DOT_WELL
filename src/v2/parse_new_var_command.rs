use crate::v1::evals::transform_to_string;
use crate::v1::string_manipulator::{separate_quoted_and_unquoted, separate_string_functions};
use crate::v1::var_maker::*;
use crate::v2::parse_values::catch_real_values;

pub fn var_manipulator(metodo: &str, resto: &str, linha: &str, pool: &mut Vec<Variable>) {
    let (nome_str, valor_str) = match resto.split_once('=') {
        Some(v) => v,
        None => {
            println!("Erro: linha sem '=' → {}", linha);
            return;
        }
    };

    let tipo_str = metodo.trim();
    let nome_str = nome_str.trim();
    let valor_str = valor_str.trim();

    if nome_str.is_empty() {
        println!("Nome da variável não pode ser vazio.");
        return;
    }

    if nome_str.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        println!("Nome da variável contém caracteres inválidos: {}", nome_str);
        return;
    }

    if nome_str.chars().next().unwrap().is_digit(10) {
        println!("Nome da variável não pode começar com número: {}", nome_str);
        return;
    }

    if find_var_value(nome_str, pool).is_ok() {
        println!("Variável '{}' já existe.", nome_str);
        return;
    }

    for variavel in pool.iter() {
        if variavel.nome == nome_str {
            println!("Variável '{}' já existe.", nome_str);
            return;
        }
    }

    let tipo = match tipo_str {
        "Bool" => Types::Bool,
        "U8" => Types::U8,
        "I8" => Types::I8,
        "U32" => Types::U32,
        "I32" => Types::I32,
        "String" => Types::String,
        _ => {
            println!("Tipo inválido: {}", tipo_str);
            return;
        }
    };

    let valor = match tipo {
        Types::Bool => {
            let response = catch_real_values(valor_str, pool, 1);

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
            let response = catch_real_values(valor_str, pool, 2);

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
            let response = catch_real_values(valor_str, pool, 3);

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
            let response = catch_real_values(valor_str, pool, 4);

            match response {
                Ok(r) => match r {
                    Values::U32(r) => Values::U32(r),
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

        Types::I32 => {
            let response = catch_real_values(valor_str, pool, 5);

            match response {
                Ok(r) => match r {
                    Values::I32(r) => Values::I32(r),
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

        Types::String => {
            if valor_str == "INSERT".to_string() {
                let mut console_input = String::new();
                println!("Insira uma string para a variável '{}': ", nome_str);
                match std::io::stdin().read_line(&mut console_input) {
                    Ok(_) => {
                        let conteudo = console_input.trim_end().to_string();
                        Values::String(conteudo)
                    }
                    Err(e) => {
                        println!("Erro ao ler entrada do console: {}", e);
                        return;
                    }
                }

                //Values::String(console_input)
            } else if valor_str.starts_with("T!(") {
                match transform_to_string(valor_str, pool) {
                    Ok(v) => Values::String(v),
                    Err(e) => {
                        println!("Erro ao transformar para String: {}", e);
                        return;
                    }
                }
            } else {
                let (quoted, unquoted) = separate_quoted_and_unquoted(valor_str);

                //println!("{quoted}, {unquoted}");
                // Deve vir entre aspas
                if (quoted.starts_with('"') && quoted.ends_with('"'))
                    || (quoted.starts_with('\'') && quoted.ends_with('\''))
                {
                    let mut conteudo = quoted[1..quoted.len() - 1].to_string();

                    //println!("{}",unquoted.len());

                    if unquoted.len() == 0 {
                        Values::String(conteudo)
                    } else {
                        if !unquoted.contains("<") || !unquoted.contains("/>") {
                            println!("Funções de string mal definidas: {}", unquoted);
                            return;
                        }

                        let functions: Vec<String> = separate_string_functions(unquoted);

                        for func in functions {
                            let func = func.as_str();
                            match func {
                                "TOUPPERCASE" => {
                                    conteudo = conteudo.to_uppercase();
                                }
                                "TOLOWERCASE" => {
                                    conteudo = conteudo.to_lowercase();
                                }
                                _ => {}
                            }
                        }

                        Values::String(conteudo)
                    }
                } else {
                    println!("String precisa estar entre aspas: {}", quoted);
                    return;
                }
            }
        }
    };
    unsafe {
        create_var(tipo, valor, nome_str, pool);
    }
}
