use crate::v1::var_maker::*;
use evalexpr::*;
use regex::Regex;

pub fn catch_real_values(s: &str, pool: &Vec<Variable>, var_type: u8) -> Result<Values, String> {
    let characters: Vec<char> = s.chars().collect();

    let mut test: String = "".to_string();

    for i in characters {
        if i.to_string() != " ".to_string() {
            test = test + &i.to_string();
        }
    }

    let delimitadores = [
        "==", "!=", ">=", "<=", "**", ">", "<", "+", "-", "*", "/", "%",
    ];
    let pattern = delimitadores
        .iter()
        .map(|d| regex::escape(d))
        .collect::<Vec<String>>()
        .join("|");
    let re = Regex::new(&pattern).unwrap();

    let valores: Vec<&str> = re.split(&test).collect();

    let response = match var_type {
        1 => {
            let texto_parseado = test.replace("true", "1");
            let mut texto_parseado = texto_parseado.replace("false", "0");
            for v in valores {
                if v == "true".to_string() || v == "false".to_string() {
                    continue;
                }

                let talvez_variavel = find_var_value(&v, pool).ok();
                let response = match talvez_variavel {
                    Some(var) => var,
                    None => v.to_string(),
                };

                texto_parseado = texto_parseado.replace(v, response.as_str());
            }

            let value = eval(&texto_parseado).map_err(|e| e.to_string())?;
            return match value {
                Value::Float(f) => Ok(Values::Bool(f != 0.0)),
                Value::Int(i) => Ok(Values::Bool(i != 0)),
                Value::Boolean(b) => Ok(Values::Bool(b)),
                _ => Err("A expressão não resultou em um valor booleano.".to_string()),
            };
        }
        2 => {
            let mut texto_parseado: String = test.clone();

            for v in valores {
                let talvez_variavel = find_var_value(&v, pool).ok();

                let response = match talvez_variavel {
                    Some(var) => var,
                    None => v.to_string(),
                };

                texto_parseado = texto_parseado.replace(v, response.as_str());
            }

            let value = eval(&texto_parseado).map_err(|e| e.to_string())?;

            return match value {
                Value::Float(f) => Ok(Values::U8(f as u8)),
                Value::Int(i) => Ok(Values::U8(i as u8)),
                _ => Err("A expressão não resultou em uma operação matemática.".to_string()),
            };
        }
        3 => {
            let mut texto_parseado: String = test.clone();

            for v in valores {
                let talvez_variavel = find_var_value(&v, pool).ok();

                let response = match talvez_variavel {
                    Some(var) => var,
                    None => v.to_string(),
                };

                texto_parseado = texto_parseado.replace(v, response.as_str());
            }

            let value = eval(&texto_parseado).map_err(|e| e.to_string())?;

            return match value {
                Value::Float(f) => Ok(Values::I8(f as i8)),
                Value::Int(i) => Ok(Values::I8(i as i8)),
                _ => Err("A expressão não resultou em uma operação matemática.".to_string()),
            };
        }
        4 => {
            let mut texto_parseado: String = test.clone();

            for v in valores {
                let talvez_variavel = find_var_value(&v, pool).ok();

                let response = match talvez_variavel {
                    Some(var) => var,
                    None => v.to_string(),
                };

                texto_parseado = texto_parseado.replace(v, response.as_str());
            }

            let value = eval(&texto_parseado).map_err(|e| e.to_string())?;

            return match value {
                Value::Float(f) => Ok(Values::U32(f as u32)),
                Value::Int(i) => Ok(Values::U32(i as u32)),
                _ => Err("A expressão não resultou em uma operação matemática.".to_string()),
            };
        }

        5 => {
            let mut texto_parseado: String = test.clone();

            for v in valores {
                let talvez_variavel = find_var_value(&v, pool).ok();

                let response = match talvez_variavel {
                    Some(var) => var,
                    None => v.to_string(),
                };

                texto_parseado = texto_parseado.replace(v, response.as_str());
            }

            let value = eval(&texto_parseado).map_err(|e| e.to_string())?;

            return match value {
                Value::Float(f) => Ok(Values::I32(f as i32)),
                Value::Int(i) => Ok(Values::I32(i as i32)),
                _ => Err("A expressão não resultou em uma operação matemática.".to_string()),
            };
        }
        _ => Err("".to_string()),
    };
    return response;
}
