use std::fs;

mod v1;

mod v2;

use v1::evals::*;
use v1::parse_var_command::var_manipulator;
use v1::prepare_text::processar_linha;
use v1::var_maker::*;
use v2::parse_new_var_command::var_manipulator as var_manipulator_2;

use v2::change_var_value::{change_var_value};
use v2::parse_values::catch_real_values;
use v2::no_method::no_method;

unsafe fn interpretar(
    linha: &str,
    pool: &mut Vec<Variable>,
    escopos: &mut Vec<Escopo>,
    ignore_next: &mut bool,
    valid_function: &mut bool,
    loop_function_checker: &mut bool,
) {
    //println!(" ----------\n Interpretando linha: {} \n", linha);

    let linha = linha.trim_end_matches(';').trim();

    if linha.ends_with("}FN") {
        *ignore_next = false;
        return;
    }

    if *ignore_next {
        let posicao = escopos.len() - 1;

        if *valid_function == false {
            return;
        }

        escopos[posicao].codigo.push_str(linha);
        escopos[posicao].codigo.push(';');
        return;
    }

    let (metodo, resto) = match linha.split_once(":") {
        Some(v) => v,
        None => {
            ("", linha)
            //println!("Método inválido → {} ", linha);
            //return;
        }
    };

    //println!("{metodo}");

    match metodo {
        "FN" => {
            *ignore_next = true;
            let (nome, codigo) = match resto.split_once('{') {
                Some(v) => v,
                None => {
                    println!("Erro: linha sem '{{' → {}", linha);
                    return;
                }
            };

            //println!("Código interno: {}", codigo);

            let escopo = Escopo {
                nome: nome.to_string(),
                codigo: codigo.trim_end_matches('}').trim().to_string() + ";",
            };

            for function in escopos.iter() {
                if function.nome == escopo.nome {
                    println!("Função '{}' já existe.", escopo.nome);
                    *valid_function = false;
                    return;
                }
            }

            *valid_function = true;

            escopos.push(escopo);

            return;
        }
        "String" | "U8" | "I8" | "U32" | "I32" | "Bool" => {
            var_manipulator_2(metodo, resto, linha, pool);
        }

        // Vou apagar isso depois, ou não também, sou indeciso
        "VAR" => {
            var_manipulator(resto, linha, pool);
        }

        "DROP" => {
            for var in pool.iter() {
                if var.nome == resto {
                    remover_var(pool, var as *const Variable);
                    //println!("Variável '{}' removida.", resto);
                    return;
                }
            }
        }

        "CHANGE" => {
            let (variavel, novo_valor) = match resto.split_once(':') {
                Some(v) => v,
                None => {
                    println!("Erro: linha sem ':' → {}", linha);
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

        "EXECUTE" => {
            println!("Chamando função: {}", resto);

            let mut codigo_funcao = None;
            for function in escopos.iter() {
                if function.nome == resto {
                    codigo_funcao = Some(function.codigo.clone());
                    break;
                }
            }

            if let Some(codigo) = codigo_funcao {
                //println!("Função encontrada! Código: {}", codigo);
                let mut foi_quebrado = false;
                for instrucao in codigo.split(';') {
                    let instrucao = instrucao.trim();
                    if instrucao.is_empty() {
                        continue;
                    }

                    unsafe {
                        interpretar(
                            instrucao,
                            pool,
                            escopos,
                            ignore_next,
                            valid_function,
                            &mut foi_quebrado,
                        );
                    }
                }
            } else {
                println!("Função '{}' não encontrada.", resto);
            }
        }
        "IF" => {
            let (condicao_str, funcao_str_) = match resto.split_once(':') {
                Some(v) => v,
                None => {
                    println!("Erro: linha sem ':' → {}", linha);
                    return;
                }
            };

            let condicao_resultado = check_condition(condicao_str, pool);

            if condicao_resultado {
                let funcao_str = funcao_str_.trim();
                //println!("Condição verdadeira, executando função: {}", funcao_str);

                let mut codigo_funcao = None;
                for function in escopos.iter() {
                    if function.nome == funcao_str {
                        codigo_funcao = Some(function.codigo.clone());
                        break;
                    }
                }

                if let Some(codigo) = codigo_funcao {
                    let mut foi_quebrado = false;
                    //println!("Função encontrada! Código: {}", codigo);
                    for instrucao in codigo.split(';') {
                        let instrucao = instrucao.trim();
                        if instrucao.is_empty() {
                            continue;
                        }
                        unsafe {
                            interpretar(
                                instrucao,
                                pool,
                                escopos,
                                ignore_next,
                                valid_function,
                                &mut foi_quebrado,
                            );
                        }
                    }
                } else {
                    if funcao_str == "BREAK".to_string() {
                        *loop_function_checker = true;
                    } else {
                        println!("Função '{}' não encontrada.", funcao_str);
                    }
                }
            } else {
                //println!("Condição falsa, não executando função: {}", funcao_str_);
            }
        }
        "WHILE" => {
            let (condicao_str, funcao_str_) = match resto.split_once(':') {
                Some(v) => v,
                None => {
                    println!("Erro: linha sem ':' → {}", linha);
                    return;
                }
            };

            let mut condicao_resultado = check_condition(condicao_str, pool);

            let mut foi_quebrado = false;

            while condicao_resultado {
                let funcao_str = funcao_str_.trim();
                //println!("Condição verdadeira, executando função: {}", funcao_str);

                let mut codigo_funcao = None;
                for function in escopos.iter() {
                    if function.nome == funcao_str {
                        codigo_funcao = Some(function.codigo.clone());
                        break;
                    }
                }

                if let Some(codigo) = codigo_funcao {
                    //println!("Função encontrada! Código: {}", codigo);
                    for instrucao in codigo.split(';') {
                        let instrucao = instrucao.trim();
                        if instrucao.is_empty() {
                            continue;
                        }

                        if foi_quebrado {
                            //println!("Quebrei o loop");
                            break;
                        }

                        unsafe {
                            interpretar(
                                instrucao,
                                pool,
                                escopos,
                                ignore_next,
                                valid_function,
                                &mut foi_quebrado,
                            );
                        }
                    }
                } else {
                    //println!("Função '{}' não encontrada.", funcao_str);
                }

                if foi_quebrado {
                    condicao_resultado = false;
                } else {
                    condicao_resultado = check_condition(condicao_str, pool);
                };
            }
        }
        "PRINT" => {
            let conteudo = resto.trim();
            match transform_to_string(conteudo, pool) {
                Ok(v) => {
                    println!("{}", v);
                }
                Err(e) => {
                    println!("Erro ao transformar para String: {}", e);
                }
            }
        }
        "" => {
            no_method(resto, pool);
        }
        _ => {}
    };
}

fn check_condition(condicao_str: &str, pool: &mut Vec<Variable>) -> bool {
    let response = catch_real_values(condicao_str, pool, 1);

    let retorno = match response {
        Ok(r) => match r {
            Values::Bool(r) => r,
            _ => {
                println!("Retorno inválido para tipo Bool");
                false
            }
        },
        Err(r) => {
            println!("Erro ao avaliar expressão: {}", r);
            false
        }
    };
    return retorno;
}

fn main() {
    let conteudo = fs::read_to_string("test.well").expect("Não foi possível ler o arquivo");

    let resultado: String = conteudo
        .lines()
        .filter(|linha| !linha.trim().is_empty())
        .map(|linha| processar_linha(linha))
        .collect();

    unsafe {
        let mut pool: Vec<Variable> = Vec::new();

        let mut escopos: Vec<Escopo> = Vec::new();

        let mut ignore_next = false;

        let mut valid_function = true;

        let mut foi_quebrado = false;

        println!(" \n\nCONSOLE OUTPUT: \n");

        for instrucao in resultado.split(';') {
            let instrucao = instrucao.trim();
            if instrucao.is_empty() {
                continue;
            }

            interpretar(
                instrucao,
                &mut pool,
                &mut escopos,
                &mut ignore_next,
                &mut valid_function,
                &mut foi_quebrado,
            );
        }

        //read_variables(&mut pool);

        // Não esquecer
        for var in &pool {
            var.destruidor();
        }
        pool.clear();
    }
}
