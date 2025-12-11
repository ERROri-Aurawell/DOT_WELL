use crate::v1::var_maker::*;
use crate::v2::change_var_value::*;

pub fn no_method(resto: &str, pool: &mut Vec<Variable>) {
    //println!("Nenhum método informado, chutando o que deveria acontecer: ");

    let characters: Vec<char> = resto.chars().collect();
    let mut resto: String = "".to_string();
    for i in &characters {
        if i.to_string() != " ".to_string() {
            resto = resto + &i.to_string();
        }
    }

    let operacoes = vec!["=", "+=", "-=", "*=", "/=", "++", "--"];
    let simbolos = vec!["=", "+", "-", "*", "/"];

    let mut operacao = "".to_string();

    let metodo: Option<String> = {
        for i in characters {
            let i = i.to_string();
            if simbolos.contains(&i.as_str()) {
                operacao = operacao + &i;
            }

            if operacao.len() > 2 {
                break;
            }
            if operacoes.contains(&operacao.as_str()) {
                break;
            }
        }
        if operacoes.contains(&operacao.as_str()) {
            Some(operacao)
        } else {
            None
        }
    };

    match metodo {
        Some(m) => {
            //println!("Método: {}", &m);
            let m = m.as_str();
            match m {
                "=" => {
                    // Nova atribuição
                    let (variavel, novo_valor) = match resto.split_once('=') {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    //println!("transformar {} em {}", variavel, novo_valor);

                    let mut position = 0;
                    for var in pool.iter() {
                        if var.nome == variavel {
                            break;
                        }
                        position += 1;
                    }

                    if position == pool.len() {
                        println!("Variável '{}' não encontrada para alteração.", variavel);
                        return;
                    }

                    change_var_value(pool, position, novo_valor);
                }
                "+=" => {
                    let (variavel, novo_valor) = match resto.split_once("+=") {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    //println!("transformar {} em {}", variavel, novo_valor);

                    let mut position = 0;
                    for var in pool.iter() {
                        if var.nome == variavel {
                            break;
                        }
                        position += 1;
                    }

                    if position == pool.len() {
                        println!("Variável '{}' não encontrada para alteração.", variavel);
                        return;
                    }

                    let novo_valor = format!("{}+{}", pool[position].nome, novo_valor);
                    let novo_valor = &novo_valor;

                    change_var_value(pool, position, novo_valor);
                }
                "-=" => {
                    let (variavel, novo_valor) = match resto.split_once("-=") {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    let mut position = 0;
                    for var in pool.iter() {
                        if var.nome == variavel {
                            break;
                        }
                        position += 1;
                    }

                    if position == pool.len() {
                        println!("Variável '{}' não encontrada para alteração.", variavel);
                        return;
                    }

                    let novo_valor = format!("{}-{}", pool[position].nome, novo_valor);
                    let novo_valor = &novo_valor;

                    change_var_value(pool, position, novo_valor);
                }
                "*=" => {
                    let (variavel, novo_valor) = match resto.split_once("*=") {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    let mut position = 0;
                    for var in pool.iter() {
                        if var.nome == variavel {
                            break;
                        }
                        position += 1;
                    }

                    if position == pool.len() {
                        println!("Variável '{}' não encontrada para alteração.", variavel);
                        return;
                    }

                    let novo_valor = format!("{} * {}", pool[position].nome, novo_valor);
                    let novo_valor = &novo_valor;

                    change_var_value(pool, position, novo_valor);
                }
                "/=" => {
                    let (variavel, novo_valor) = match resto.split_once("/=") {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    let mut position = 0;
                    for var in pool.iter() {
                        if var.nome == variavel {
                            break;
                        }
                        position += 1;
                    }

                    if position == pool.len() {
                        println!("Variável '{}' não encontrada para alteração.", variavel);
                        return;
                    }

                    let novo_valor = format!("{} / {}", pool[position].nome, novo_valor);
                    let novo_valor = &novo_valor;

                    change_var_value(pool, position, novo_valor);
                }
                "++" => {
                    let (variavel, _) = match resto.split_once("++") {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    let mut position = 0;
                    for var in pool.iter() {
                        if var.nome == variavel {
                            break;
                        }
                        position += 1;
                    }

                    if position == pool.len() {
                        println!("Variável '{}' não encontrada para alteração.", variavel);
                        return;
                    }

                    soma_1(pool, position);
                }
                "--" => {
                    let (variavel, _) = match resto.split_once("--") {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    let mut position = 0;
                    for var in pool.iter() {
                        if var.nome == variavel {
                            break;
                        }
                        position += 1;
                    }

                    if position == pool.len() {
                        println!("Variável '{}' não encontrada para alteração.", variavel);
                        return;
                    }

                    subtrai_1(pool, position);
                }
                _ => {}
            }
        }
        None => {
            println!("Método não reconhecido em → {}", resto);
        }
    }
}
