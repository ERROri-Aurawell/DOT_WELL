use std::fs;

use std::cell::RefCell;
use std::rc::Rc;

mod v1;
mod v2;

use v1::evals::*;
use v1::parse_var_command::var_manipulator;
use v1::prepare_text::processar_linha;
use v1::var_maker::*;
use v2::parse_new_var_command::var_manipulator as var_manipulator_2;

use v2::change_var_value::change_var_value;
use v2::function_props::{function_props, montar_funcao};
use v2::no_method::no_method;
use v2::parse_values::catch_real_values;

struct Escopo {
    superior: Option<Rc<RefCell<Escopo>>>,
    atual: Vec<Variable>,
}

impl Escopo {
    pub fn buscar(&self, nome: &str) -> Option<Variable> {
        self.atual
            .iter()
            .find(|v| v.nome == nome)
            .map(|v| Variable {
                nome: v.nome.clone(),
                valor: v.valor.clone(),
                tipo: v.tipo.clone(),
                ponteiro: v.ponteiro.clone(),
            })
            .or_else(|| self.superior.as_ref()?.borrow().buscar(nome))
    }
}

impl Escopo {
    fn novo_filho(pai: Rc<RefCell<Escopo>>) -> Rc<RefCell<Escopo>> {
        Rc::new(RefCell::new(Escopo {
            superior: Some(pai),
            atual: Vec::new(),
        }))
    }
}

pub unsafe fn interpretar(
    linha: &str,
    pool: &mut Vec<Variable>,
    funcoes: &mut Vec<Funcao>,
    loop_function_checker: &mut bool,
    function_pr: &mut function_props,
    escopo_global: &mut Rc<RefCell<Escopo>>,
) {
    let linha = linha.trim_end_matches(';').trim();

    if function_pr.em_construcao {
        montar_funcao(linha, function_pr, funcoes);
        return;
    }

    //println!("----------\nInterpretando linha: {} \n", linha);

    let (metodo, resto) = match linha.split_once(":") {
        Some(v) => v,
        None => ("", linha),
    };

    match metodo {
        "FN" => {
            //println!("ENTROU NO ESCOPO -----------");
            function_pr.em_construcao = true;

            let (function_name, resto) = match resto.split_once("(") {
                Some(v) => v,
                None => ("", resto),
            };

            for function in funcoes.iter() {
                if function.nome == function_name {
                    println!("Erro: Já existe uma função nomeada: '{}'", function_name);
                    return;
                }
            }

            let (parameters, codigo) = match resto.split_once(")") {
                Some(v) => v,
                None => ("", resto),
            };

            let escopo = Funcao {
                nome: function_name.to_string(),
                codigo: "".to_string(),
                parameters: parameters.split(",").map(|s| s.to_string()).collect(),
            };
            funcoes.push(escopo);

            montar_funcao(codigo, function_pr, funcoes);
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

            let mut codigo_funcao: Option<String> = None;

            //let mut parametros_funct: Option<Vec<String>> = None; //isso vai ser usado quando eu implementar escopo

            for function in funcoes.iter() {
                if function.nome == resto {
                    codigo_funcao = Some(function.codigo.clone());
                    //parametros_funct = Some(function.parameters.clone());
                    break;
                }
            }

            if let Some(codigo) = codigo_funcao {
                //println!("Função encontrada! Código: {}", codigo);
                let mut function_pr = function_props {
                    em_construcao: false,
                    ignorar_chaves: 0,
                };
                for instrucao in codigo.split(';') {
                    let instrucao = instrucao.trim();
                    if instrucao.is_empty() {
                        continue;
                    }

                    let mut escopo_interno = Escopo::novo_filho(escopo_global.clone());

                    unsafe {
                        interpretar(
                            instrucao,
                            pool,
                            funcoes,
                            loop_function_checker,
                            &mut function_pr,
                            &mut escopo_interno,
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
                for function in funcoes.iter() {
                    if function.nome == funcao_str {
                        codigo_funcao = Some(function.codigo.clone());
                        break;
                    }
                }

                if let Some(codigo) = codigo_funcao {
                    //println!("Função encontrada! Código: {}", codigo);
                    let mut function_pr = function_props {
                        em_construcao: false,
                        ignorar_chaves: 0,
                    };
                    let mut escopo_interno = Escopo::novo_filho(escopo_global.clone());

                    for instrucao in codigo.split(';') {
                        let instrucao = instrucao.trim();
                        if instrucao.is_empty() {
                            continue;
                        }
                        unsafe {
                            interpretar(
                                instrucao,
                                pool,
                                funcoes,
                                loop_function_checker,
                                &mut function_pr,
                                &mut escopo_interno,
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
                for function in funcoes.iter() {
                    if function.nome == funcao_str {
                        codigo_funcao = Some(function.codigo.clone());
                        break;
                    }
                }

                if let Some(codigo) = codigo_funcao {
                    //println!("Função encontrada! Código: {}", codigo);
                    let mut function_pr = function_props {
                        em_construcao: false,
                        ignorar_chaves: 0,
                    };
                    let mut escopo_interno = Escopo::novo_filho(escopo_global.clone());
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
                                funcoes,
                                loop_function_checker,
                                &mut function_pr,
                                &mut escopo_interno,
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
            let (nome, inner) = match resto.split_once("(") {
                Some(s) => s,
                None => {
                    println!("");
                    ("", "")
                }
            };
            if nome == "" {
                no_method(resto, pool);
                return;
            }

            let parametros: Vec<String> = inner
                .split(',')
                .map(|s| s.chars().filter(|&c| c.is_alphanumeric()).collect())
                .collect();

            let mut codigo_funcao: Option<String> = None;
            let mut parametros_funct: Option<Vec<String>> = None; //isso vai ser usado quando eu implementar escopo

            for function in funcoes.iter() {
                if function.nome == nome {
                    codigo_funcao = Some(function.codigo.clone());
                    parametros_funct = Some(function.parameters.clone());
                    break;
                }
            }
            if let Some(_parameters) = parametros_funct {
                if parametros.len() != _parameters.len() {
                    println!(
                        "Erro: Função '{}' esperava {} parametros",
                        nome,
                        _parameters.len()
                    );
                    return;
                }
                for (indice, par) in _parameters.iter().enumerate() {
                    println!(
                        "Parâmetro: {} - índice {} - recebe {}",
                        par, indice, parametros[indice]
                    );
                }
            }

            if let Some(codigo) = codigo_funcao {
                //println!("Função encontrada! Código: {}", codigo);
                let mut function_pr = function_props {
                    em_construcao: false,
                    ignorar_chaves: 0,
                };
                let mut escopo_interno = Escopo::novo_filho(escopo_global.clone());
                for instrucao in codigo.split(';') {
                    let instrucao = instrucao.trim();
                    if instrucao.is_empty() {
                        continue;
                    }

                    unsafe {
                        interpretar(
                            instrucao,
                            pool,
                            funcoes,
                            loop_function_checker,
                            &mut function_pr,
                            &mut escopo_interno,
                        );
                    }
                }
                return;
            };
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

        let mut funcoes: Vec<Funcao> = Vec::new();
        let mut loop_function_checker = false;

        let mut function_pr = function_props {
            em_construcao: false,
            ignorar_chaves: 0,
        };

        println!(" \n\nCONSOLE OUTPUT: \n");

        let mut escopo = Rc::new(RefCell::new(Escopo {
            superior: None,
            atual: Vec::new(),
        }));

        for instrucao in resultado.split(';') {
            let linha = instrucao.trim();
            if linha.is_empty() {
                continue;
            }

            interpretar(
                linha,
                &mut pool,
                &mut funcoes,
                &mut loop_function_checker,
                &mut function_pr,
                &mut escopo,
            );
        }

        //read_variables(&mut pool);

        // Não esquecer
        for var in &pool {
            var.destruidor();
        }
        pool.clear();

        /*
        for var in &escopo.atual {
            var.destruidor();
        }
        escopo.atual.clear();
        */
    }
}
