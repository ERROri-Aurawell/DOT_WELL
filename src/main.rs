use std::fs;

use std::cell::RefCell;
use std::rc::Rc;

mod v1;
mod v2;

//use v1::evals::*;
//use v1::parse_var_command::var_manipulator;
use v1::prepare_text::processar_linha;
use v1::var_maker::*;
use v2::parse_new_var_command::var_manipulator as var_manipulator_2;

use v2::function_props::{function_props, montar_funcao};
use v2::no_method::no_method;
use v2::parse_values::catch_real_values;

pub struct Escopo {
    superior: Option<Rc<RefCell<Escopo>>>,
    dados: Vec<Variable>,
}

impl Escopo {
    pub fn buscar(&self, nome: &str) -> Option<Variable> {
        self.dados
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
            dados: Vec::new(),
        }))
    }
}

impl Escopo {
    pub fn create_var(&mut self, tipo: Types, valor: Values, nome: &str) {
        let var = Variable {
            tipo,
            ponteiro: Pointers::from_value(&valor),
            valor,
            nome: nome.to_string(),
        };

        self.dados.push(var);
    }
}

impl Escopo {
    pub fn remover_var(&mut self, nome: &str) -> bool {
        if let Some(pos) = self.dados.iter().position(|v| v.nome == nome) {
            self.dados.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Escopo {
    pub fn atribuir(&mut self, nome: &str, valor: Values) -> bool {
        if let Some(v) = self.dados.iter_mut().find(|v| v.nome == nome) {
            v.valor = valor;
            true
        } else if let Some(pai) = &self.superior {
            pai.borrow_mut().atribuir(nome, valor)
        } else {
            false
        }
    }
}

unsafe fn interpretar(
    linha: &str,
    pool: &mut Vec<Variable>,
    funcoes: &mut Vec<Funcao>,
    loop_function_checker: &mut bool,
    function_pr: &mut function_props,
    escopo_atual: &mut Rc<RefCell<Escopo>>,
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
            var_manipulator_2(metodo, resto, linha, escopo_atual);
        }

        // Vou apagar isso depois, ou não também, sou indeciso
        "VAR" => {
            //var_manipulator(resto, linha, pool);
        }

        "DROP" => {
            escopo_atual.borrow_mut().remover_var(resto);
            /*
            for var in pool.iter() {
                if var.nome == resto {
                    remover_var(pool, var as *const Variable);
                    //println!("Variável '{}' removida.", resto);
                    return;
                }
            }
            */
        }

        "CHANGE" => {
            let (variavel, novo_valor) = match resto.split_once(':') {
                Some(v) => v,
                None => {
                    println!("Erro: linha sem ':' → {}", linha);
                    return;
                }
            };

            // 1️⃣ buscar variável existente (para pegar o tipo)
            let var_existente = match escopo_atual.borrow().buscar(variavel) {
                Some(v) => v,
                None => {
                    println!("Variável '{}' não encontrada para alteração.", variavel);
                    return;
                }
            };

            // 2️⃣ avaliar expressão com base no tipo da variável
            let valor = match catch_real_values(novo_valor, escopo_atual, var_existente.tipo as u8)
            {
                Ok(v) => v,
                Err(e) => {
                    println!("Erro ao avaliar '{}': {}", novo_valor, e);
                    return;
                }
            };

            // 3️⃣ atribuir
            escopo_atual.borrow_mut().atribuir(variavel, valor);
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

                    let mut escopo_interno = Escopo::novo_filho(escopo_atual.clone());

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

            let condicao_resultado = check_condition(condicao_str, escopo_atual);

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
                    let mut escopo_interno = Escopo::novo_filho(escopo_atual.clone());

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

            let mut condicao_resultado = check_condition(condicao_str, escopo_atual);

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
                    let mut escopo_interno = Escopo::novo_filho(escopo_atual.clone());
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
                    condicao_resultado = check_condition(condicao_str, escopo_atual);
                };
            }
        }
        "PRINT" => {
            //DEPOIS EU ARRUMO O PRINT

            //println!("{}", resto);

            // 1️⃣ buscar variável existente (para pegar o tipo)
            let var_existente = match escopo_atual.borrow().buscar(resto) {
                Some(v) => v,
                None => {
                    println!("Variável '{}' não encontrada para alteração.", resto);
                    return;
                }
            };

            let novo_valor = format!("{resto} - 0");

            let novo_valor = &novo_valor;

            // 2️⃣ avaliar expressão com base no tipo da variável
            let valor = match catch_real_values(novo_valor, escopo_atual, var_existente.tipo as u8)
            {
                Ok(v) => v,
                Err(e) => {
                    println!("Erro ao avaliar '{}': {}", novo_valor, e);
                    return;
                }
            };

            match valor {
                Values::U8(v) => {
                    println!("{v}")
                }
                _ => {
                    println!("Deu erro?")
                }
            }

            /*
            let conteudo = resto.trim();
            match transform_to_string(conteudo, escopo_atual) {
                Ok(v) => {
                    println!("{}", v);
                }
                Err(e) => {
                    println!("Erro ao transformar para String: {}", e);
                }
            }
            */
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
                no_method(resto, escopo_atual);
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
                let mut escopo_interno = Escopo::novo_filho(escopo_atual.clone());
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

fn check_condition(condicao_str: &str, escopo_atual: &mut Rc<RefCell<Escopo>>) -> bool {
    let response = match catch_real_values(condicao_str, escopo_atual, 1) {
        Ok(v) => v,
        Err(e) => {
            println!("Erro ao avaliar: {e}");
            return false;
        }
    };

    let retorno = match response {
        Values::Bool(r) => r,
        _ => {
            println!("Retorno inválido para tipo Bool");
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
            dados: Vec::new(),
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
