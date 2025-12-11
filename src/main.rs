use std::fs;

mod v1;

mod v2;

use v1::evals::*;
use v1::parse_var_command::var_manipulator;
use v1::prepare_text::processar_linha;
use v1::string_manipulator::separate_quoted_and_unquoted;
use v1::string_manipulator::*;
use v1::var_maker::*;

use v2::change_var_value::change_var_value;
use v2::parse_values::catch_real_values;

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

            create_var(tipo, valor, nome_str, pool);
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
                println!("Condição verdadeira, executando função: {}", funcao_str);

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
            println!("Nenhum método informado, chutando o que deveria acontecer: ");

            let characters: Vec<char> = resto.chars().collect();
            let mut resto: String = "".to_string();
            for i in characters {
                if i.to_string() != " ".to_string() {
                    resto = resto + &i.to_string();
                }
            }

            let (variavel, novo_valor) = match resto.split_once('=') {
                Some(v) => v,
                None => {
                    return;
                }
            };

            println!("transformar {} em {}", variavel, novo_valor);

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
