use crate::v1::extract_macros::{
    extract_b_macro, extract_m_macro, extract_t_macro, extract_v_macro,
};

use evalexpr::*;

use crate::v1::var_maker::*;

pub fn eval_m(s: &str, pool: &Vec<Variable>) -> Result<f64, String> {
    let mut result = s.to_string();

    while let Some((start, end, var_name)) = extract_v_macro(&result) {
        let var_value = find_var_value(&var_name, pool)?;
        result.replace_range(start..end, &var_value);
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

pub fn transform_to_string(s: &str, pool: &Vec<Variable>) -> Result<String, String> {
    let mut result = s.to_string();

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

    Ok(result)
}
