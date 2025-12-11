use crate::v1::evals::*;
use crate::v1::string_manipulator::{separate_quoted_and_unquoted, separate_string_functions};
use crate::v1::var_maker::*;

pub fn var_manipulator(resto: &str, linha: &str, pool: &mut Vec<Variable>) {
    let (tipo_str, resto) = match resto.split_once(':') {
        Some(v) => v,
        None => {
            println!("Erro: linha sem ':' → {}", linha);
            return;
        }
    };

    let (nome_str, valor_str) = match resto.split_once('=') {
        Some(v) => v,
        None => {
            println!("Erro: linha sem '=' → {}", linha);
            return;
        }
    };

    let tipo_str = tipo_str.trim();
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
            if valor_str.starts_with("V!(") || valor_str.starts_with("B!(") {
                match eval_b(valor_str, pool) {
                    Ok(v) => Values::Bool(v),
                    Err(e) => {
                        println!("Erro ao avaliar expressão: {}", e);
                        return;
                    }
                }
            } else {
                match valor_str {
                    "true" => Values::Bool(true),
                    "false" => Values::Bool(false),
                    _ => {
                        println!("Valor inválido para Bool: {}", valor_str);
                        return;
                    }
                }
            }
        }

        Types::U8 => {
            if valor_str.starts_with("M!(") || valor_str.starts_with("V!(") {
                match eval_m(valor_str, pool) {
                    Ok(v) => Values::U8(v as u8),
                    Err(e) => {
                        println!("Erro ao avaliar expressão: {}", e);
                        return;
                    }
                }
            } else {
                match valor_str.parse::<u8>() {
                    Ok(v) => Values::U8(v),
                    Err(e) => {
                        println!("Erro ao converter valor para U8: {}", e);
                        return;
                    }
                }
            }
        }

        Types::I8 => {
            if valor_str.starts_with("M!(") || valor_str.starts_with("V!(") {
                match eval_m(valor_str, pool) {
                    Ok(v) => Values::I8(v as i8),
                    Err(e) => {
                        println!("Erro ao avaliar expressão: {}", e);
                        return;
                    }
                }
            } else {
                match valor_str.parse::<i8>() {
                    Ok(v) => Values::I8(v),
                    Err(e) => {
                        println!("Erro ao converter valor para I8: {}", e);
                        return;
                    }
                }
            }
        }

        Types::U32 => {
            if valor_str.starts_with("M!(") || valor_str.starts_with("V!(") {
                match eval_m(valor_str, pool) {
                    Ok(v) => Values::U32(v as u32),
                    Err(e) => {
                        println!("Erro ao avaliar expressão: {}", e);
                        return;
                    }
                }
            } else {
                match valor_str.parse::<u32>() {
                    Ok(v) => Values::U32(v),
                    Err(e) => {
                        println!("Erro ao converter valor para U32: {}", e);
                        return;
                    }
                }
            }
        }

        Types::I32 => {
            if valor_str.starts_with("M!(") || valor_str.starts_with("V!(") {
                match eval_m(valor_str, pool) {
                    Ok(v) => Values::I32(v as i32),
                    Err(e) => {
                        println!("Erro ao avaliar expressão: {}", e);
                        return;
                    }
                }
            } else {
                match valor_str.parse::<i32>() {
                    Ok(v) => Values::I32(v),
                    Err(e) => {
                        println!("Erro ao converter valor para I32: {}", e);
                        return;
                    }
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
    };
}
