use crate::Escopo;
use crate::v1::extract_macros::{
    extract_b_macro, extract_m_macro, extract_t_macro, extract_v_macro,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::v1::var_maker::*;
use crate::v2::parse_values::catch_real_values;
use evalexpr::*;

/*
pub fn eval_m(s: &str, variaveis: &mut Rc<RefCell<Escopo>>) -> Result<f64, String> {
    let mut result = s.to_string();

    while let Some((start, end, var_name)) = extract_v_macro(&result) {
        //let var_value = find_var_value(&var_name, pool)?;
        let var_existente = variaveis.borrow().buscar(&var_name);
        match var_existente {
            Some(r) => {
                result.replace_range(start..end, &);
            }
            None =>{}
        }
    }

    loop {
        if let Some((start, end, content)) = extract_m_macro(&result) {
            let inner_value = eval_m(&content, pool)?;

            result.replace_range(start..end, &inner_value.to_string());
        } else {
            let value = eval(&result).map_err(|e| e.to_string())?;
            return match value {
                Value::Float(f) => Ok(f),
                Value::Int(i) => Ok(i as f64),
                _ => Err("A expressão não resultou em um número.".to_string()),
            };
        }
    }
}

pub fn eval_b(s: &str, pool: &Vec<Variable>) -> Result<bool, String> {
    let mut result = s.to_string();

    while let Some((start, end, var_name)) = extract_v_macro(&result) {
        let var_value = find_var_value(&var_name, pool)?;
        result.replace_range(start..end, &var_value);
    }

    while let Some((start, end, content)) = extract_m_macro(&result) {
        let inner_value = eval_m(&content, pool)?;
        result.replace_range(start..end, &inner_value.to_string());
    }

    loop {
        if let Some((start, end, content)) = extract_b_macro(&result) {
            let inner_value = eval_b(&content, pool)?;
            result.replace_range(start..end, if inner_value { "1" } else { "0" });
        } else {
            result = result.replace("true", "1");
            result = result.replace("false", "0");

            let value = eval(&result).map_err(|e| e.to_string())?;
            return match value {
                Value::Float(f) => Ok(f != 0.0),
                Value::Int(i) => Ok(i != 0),
                Value::Boolean(b) => Ok(b),
                _ => Err("A expressão não resultou em um valor booleano.".to_string()),
            };
        }
    }
}
*/

pub fn transform_to_string(s: &str, vars: &mut Rc<RefCell<Escopo>>) -> Result<String, String> {
    let mut result = s.to_string();

    // 1️⃣ buscar variável existente (para pegar o tipo)
    let var_existente = match vars.borrow().buscar(s) {
        Some(v) => v,
        None => {
            // If the variable is not found, assume it's a literal string or an expression that doesn't involve a direct variable lookup for its type.
            return catch_real_values(s, vars, 6).map(|val| match val { Values::String(s) => s, _ => s.to_string() }); // Assuming 6 is the type for String
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

    Ok(match valor { Values::String(s) => s, _ => valor.to_string() })

    /*
    while let Some((start, end, var_name)) = extract_v_macro(&result) {
        let var_value = find_var_value(&var_name, pool)?;
        result.replace_range(start..end, &var_value);
    }

    while let Some((start, end, content)) = extract_m_macro(&result) {
        let inner_value = eval_m(&content, pool)?;
        result.replace_range(start..end, &inner_value.to_string());
    }

    while let Some((start, end, content)) = extract_b_macro(&result) {
        let inner_value = eval_b(&content, pool)?;
        result.replace_range(start..end, if inner_value { "true" } else { "false" });
    }

    while let Some((start, end, content)) = extract_t_macro(&result) {
        let inner_value = transform_to_string(&content, pool)?;
        result.replace_range(start..end, &inner_value);
    }
    //println!("Transformado para String: {}", result);
    */

}
