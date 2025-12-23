use crate::v1::var_maker::*;

use crate::v2::parse_values::catch_real_values;

use crate::Escopo;
use std::cell::RefCell;
use std::rc::Rc;

pub fn change_var_value(variaveis: &mut Rc<RefCell<Escopo>>, nome: &str, novo_valor: &str) {
    // 1️⃣ buscar variável existente (para pegar o tipo)
    let var_existente = match variaveis.borrow().buscar(nome) {
        Some(v) => v,
        None => {
            println!("Variável '{}' não encontrada para alteração.", nome);
            return;
        }
    };

    // 2️⃣ avaliar expressão com base no tipo da variável
    let valor = match catch_real_values(novo_valor, variaveis, var_existente.tipo as u8) {
        Ok(v) => v,
        Err(e) => {
            println!("Erro ao avaliar '{}': {}", novo_valor, e);
            return;
        }
    };

    // 3️⃣ atribuir
    variaveis.borrow_mut().atribuir(nome, valor);
}

pub fn soma_1(variaveis: &mut Rc<RefCell<Escopo>>, nome: &str) {
    // 1️⃣ buscar variável existente (para pegar o tipo)
    let var_existente = match variaveis.borrow().buscar(nome) {
        Some(v) => v,
        None => {
            println!("Variável '{}' não encontrada para alteração.", nome);
            return;
        }
    };

    let novo_valor = format!("{nome} + 1");

    let novo_valor = &novo_valor;

    // 2️⃣ avaliar expressão com base no tipo da variável
    let valor = match catch_real_values(novo_valor, variaveis, var_existente.tipo as u8) {
        Ok(v) => v,
        Err(e) => {
            println!("Erro ao avaliar '{}': {}", novo_valor, e);
            return;
        }
    };

    // 3️⃣ atribuir
    variaveis.borrow_mut().atribuir(nome, valor);
}

pub fn subtrai_1(variaveis: &mut Rc<RefCell<Escopo>>, nome: &str) {
    // 1️⃣ buscar variável existente (para pegar o tipo)
    let var_existente = match variaveis.borrow().buscar(nome) {
        Some(v) => v,
        None => {
            println!("Variável '{}' não encontrada para alteração.", nome);
            return;
        }
    };

    let novo_valor = format!("{nome} - 1");

    let novo_valor = &novo_valor;

    // 2️⃣ avaliar expressão com base no tipo da variável
    let valor = match catch_real_values(novo_valor, variaveis, var_existente.tipo as u8) {
        Ok(v) => v,
        Err(e) => {
            println!("Erro ao avaliar '{}': {}", novo_valor, e);
            return;
        }
    };

    // 3️⃣ atribuir
    variaveis.borrow_mut().atribuir(nome, valor);
}
