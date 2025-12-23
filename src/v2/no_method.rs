use crate::Escopo;
use crate::v1::var_maker::*;
use crate::v2::change_var_value::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn no_method(resto: &str, variaveis: &mut Rc<RefCell<Escopo>>) {
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

                    //"variavel" é o nome da tal

                    change_var_value(variaveis, variavel, novo_valor);
                }
                "+=" => {
                    let (variavel, novo_valor) = match resto.split_once("+=") {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    let novo_valor = format!("{}+{}", variavel, novo_valor);
                    let novo_valor = &novo_valor;

                    change_var_value(variaveis, variavel, novo_valor);
                }
                "-=" => {
                    let (variavel, novo_valor) = match resto.split_once("-=") {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    let novo_valor = format!("{}-{}", variavel, novo_valor);
                    let novo_valor = &novo_valor;

                    change_var_value(variaveis, variavel, novo_valor);
                }
                "*=" => {
                    let (variavel, novo_valor) = match resto.split_once("*=") {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    let novo_valor = format!("{} * {}", variavel, novo_valor);
                    let novo_valor = &novo_valor;

                    change_var_value(variaveis, variavel, novo_valor);
                }
                "/=" => {
                    let (variavel, novo_valor) = match resto.split_once("/=") {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    let novo_valor = format!("{} / {}", variavel, novo_valor);
                    let novo_valor = &novo_valor;

                    change_var_value(variaveis, variavel, novo_valor);
                }
                "++" => {
                    let (variavel, _) = match resto.split_once("++") {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    soma_1(variaveis, variavel);
                }
                "--" => {
                    let (variavel, _) = match resto.split_once("--") {
                        Some(v) => v,
                        None => {
                            return;
                        }
                    };

                    subtrai_1(variaveis, variavel);
                }
                _ => {}
            }
        }
        None => {
            println!("Método não reconhecido em → {}", resto);
        }
    }
}
