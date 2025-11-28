use evalexpr::*;
use std::fs;

use std::ptr;

enum Types {
    Bool,
    U8,
    I8,
    U32,
    I32,
    String,
}

enum Values {
    Bool(bool),
    U8(u8),
    I8(i8),
    U32(u32),
    I32(i32),
    String(String),
}

enum Pointers {
    Bool(*mut bool),
    U8(*mut u8),
    I8(*mut i8),
    U32(*mut u32),
    I32(*mut i32),
    String(*mut String),
}
struct Variable {
    valor: Values,
    tipo: Types,
    ponteiro: Pointers,
    nome: String,
}

struct Escopo {
    nome: String,
    codigo: String,
}

impl Variable {
    unsafe fn destruidor(&self) {
        match self.ponteiro {
            Pointers::Bool(ptr) => ptr::drop_in_place(ptr),
            Pointers::U8(ptr) => ptr::drop_in_place(ptr),
            Pointers::I8(ptr) => ptr::drop_in_place(ptr),
            Pointers::U32(ptr) => ptr::drop_in_place(ptr),
            Pointers::I32(ptr) => ptr::drop_in_place(ptr),
            Pointers::String(ptr) => ptr::drop_in_place(ptr),
        }
    }
}

unsafe fn create_var(
    tipo: Types,
    valor: Values,
    nome: &str,
    pool: &mut Vec<Variable>,
) -> *const Variable {
    let ponteiro = match &valor {
        Values::Bool(v) => Pointers::Bool(Box::into_raw(Box::new(*v))),
        Values::U8(v) => Pointers::U8(Box::into_raw(Box::new(*v))),
        Values::I8(v) => Pointers::I8(Box::into_raw(Box::new(*v))),
        Values::U32(v) => Pointers::U32(Box::into_raw(Box::new(*v))),
        Values::I32(v) => Pointers::I32(Box::into_raw(Box::new(*v))),
        Values::String(v) => Pointers::String(Box::into_raw(Box::new(v.clone()))),
    };

    let var = Variable {
        valor,
        tipo,
        ponteiro,
        nome: nome.to_string(),
    };

    pool.push(var);

    pool.last().unwrap() as *const Variable
}

unsafe fn remover_var(pool: &mut Vec<Variable>, var_ptr: *const Variable) {
    if let Some(pos) = pool.iter().position(|v| v as *const Variable == var_ptr) {
        match pool[pos].ponteiro {
            Pointers::Bool(ptr) => {
                let _ = Box::from_raw(ptr);
            }
            Pointers::U8(ptr) => {
                let _ = Box::from_raw(ptr);
            }
            Pointers::I8(ptr) => {
                let _ = Box::from_raw(ptr);
            }
            Pointers::U32(ptr) => {
                let _ = Box::from_raw(ptr);
            }
            Pointers::I32(ptr) => {
                let _ = Box::from_raw(ptr);
            }
            Pointers::String(ptr) => {
                let _ = Box::from_raw(ptr);
            }
        }

        pool.remove(pos);
    }
}

fn processar_linha(linha: &str) -> String {
    //println!("Processando linha: {}", linha);
    let mut resultado = String::new();
    let mut chars = linha.chars().peekable();

    let mut dentro_string = false;
    let mut asp = '\0';
    let mut escape = false;

    while let Some(c) = chars.next() {
        if dentro_string {
            if escape {
                resultado.push(c);
                escape = false;
                continue;
            }

            if c == '\\' {
                escape = true;
                resultado.push(c);
                continue;
            }

            if c == asp {
                dentro_string = false;
                resultado.push(c);
                continue;
            }

            resultado.push(c);
            continue;
        }

        if c == '"' || c == '\'' {
            dentro_string = true;
            asp = c;
            resultado.push(c);
            continue;
        }

        if c == '/' {
            if let Some('/') = chars.peek() {
                break;
            }
        }

        // if c == ' ' || c == '\t' {
        //   continue;
        //}

        resultado.push(c);
    }

    resultado
}

fn extract_m_macro(s: &str) -> Option<(usize, usize, String)> {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i + 2 < bytes.len() {
        if bytes[i] == b'M' && bytes[i + 1] == b'!' && bytes[i + 2] == b'(' {
            let start_content = i + 3;
            let mut depth = 1;
            let mut j = start_content;

            while j < bytes.len() {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => {
                        depth -= 1;
                        if depth == 0 {
                            let end_content = j;
                            let content = s[start_content..end_content].to_string();
                            return Some((i, j + 1, content));
                        }
                    }
                    _ => {}
                }
                j += 1;
            }

            return None;
        }

        i += 1;
    }

    None
}

fn extract_v_macro(s: &str) -> Option<(usize, usize, String)> {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i + 2 < bytes.len() {
        if bytes[i] == b'V' && bytes[i + 1] == b'!' && bytes[i + 2] == b'(' {
            let start_content = i + 3;
            let mut depth = 1;
            let mut j = start_content;

            while j < bytes.len() {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => {
                        depth -= 1;
                        if depth == 0 {
                            let end_content = j;
                            let content = s[start_content..end_content].to_string();
                            return Some((i, j + 1, content));
                        }
                    }
                    _ => {}
                }
                j += 1;
            }

            return None;
        }

        i += 1;
    }

    None
}

fn extract_b_macro(s: &str) -> Option<(usize, usize, String)> {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i + 2 < bytes.len() {
        if bytes[i] == b'B' && bytes[i + 1] == b'!' && bytes[i + 2] == b'(' {
            let start_content = i + 3;
            let mut depth = 1;
            let mut j = start_content;

            while j < bytes.len() {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => {
                        depth -= 1;
                        if depth == 0 {
                            let end_content = j;
                            let content = s[start_content..end_content].trim().to_string();
                            return Some((i, j + 1, content));
                        }
                    }
                    _ => {}
                }
                j += 1;
            }

            return None;
        }

        i += 1;
    }

    None
}

fn extract_t_macro(s: &str) -> Option<(usize, usize, String)> {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i + 2 < bytes.len() {
        if bytes[i] == b'T' && bytes[i + 1] == b'!' && bytes[i + 2] == b'(' {
            let start_content = i + 3;
            let mut depth = 1;
            let mut j = start_content;

            while j < bytes.len() {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => {
                        depth -= 1;
                        if depth == 0 {
                            let end_content = j;
                            let content = s[start_content..end_content].trim().to_string();
                            return Some((i, j + 1, content));
                        }
                    }
                    _ => {}
                }
                j += 1;
            }

            return None;
        }

        i += 1;
    }

    None
}

fn find_var_value(name: &str, pool: &Vec<Variable>) -> Result<String, String> {
    for var in pool {
        if var.nome == name {
            return Ok(match &var.valor {
                Values::Bool(v) => (if *v { 1 } else { 0 }).to_string(),
                Values::U8(v) => v.to_string(),
                Values::I8(v) => v.to_string(),
                Values::U32(v) => v.to_string(),
                Values::I32(v) => v.to_string(),
                Values::String(v) => v.clone(),
            });
        }
    }
    Err(format!("Variável '{}' não encontrada.", name))
}

fn eval_m(s: &str, pool: &Vec<Variable>) -> Result<f64, String> {
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

fn eval_b(s: &str, pool: &Vec<Variable>) -> Result<bool, String> {
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

fn transform_to_string(s: &str, pool: &Vec<Variable>) -> Result<String, String> {
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
            println!("Método inválido → {} ", linha);
            return;
        }
    };

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

            println!("Código interno: {}", codigo);

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
        "VAR" => {
            let (tipo_str, resto) = match resto.split_once(':') {
                Some(v) => v,
                None => {
                    println!("Erro: linha sem ':' → {}", linha);
                    return;
                }
            };

            let (nome_str, valor_str) = match resto.split_once('=') {
                Some(v) => v,
                None => {
                    println!("Erro: linha sem '=' → {}", linha);
                    return;
                }
            };

            let tipo_str = tipo_str.trim();
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
                    if valor_str.starts_with("V!(") || valor_str.starts_with("B!(") {
                        match eval_b(valor_str, pool) {
                            Ok(v) => Values::Bool(v),
                            Err(e) => {
                                println!("Erro ao avaliar expressão: {}", e);
                                return;
                            }
                        }
                    } else {
                        match valor_str {
                            "true" => Values::Bool(true),
                            "false" => Values::Bool(false),
                            _ => {
                                println!("Valor inválido para Bool: {}", valor_str);
                                return;
                            }
                        }
                    }
                }

                Types::U8 => {
                    if valor_str.starts_with("M!(") || valor_str.starts_with("V!(") {
                        match eval_m(valor_str, pool) {
                            Ok(v) => Values::U8(v as u8),
                            Err(e) => {
                                println!("Erro ao avaliar expressão: {}", e);
                                return;
                            }
                        }
                    } else {
                        match valor_str.parse::<u8>() {
                            Ok(v) => Values::U8(v),
                            Err(e) => {
                                println!("Erro ao converter valor para U8: {}", e);
                                return;
                            }
                        }
                    }
                }

                Types::I8 => {
                    if valor_str.starts_with("M!(") || valor_str.starts_with("V!(") {
                        match eval_m(valor_str, pool) {
                            Ok(v) => Values::I8(v as i8),
                            Err(e) => {
                                println!("Erro ao avaliar expressão: {}", e);
                                return;
                            }
                        }
                    } else {
                        match valor_str.parse::<i8>() {
                            Ok(v) => Values::I8(v),
                            Err(e) => {
                                println!("Erro ao converter valor para I8: {}", e);
                                return;
                            }
                        }
                    }
                }

                Types::U32 => {
                    if valor_str.starts_with("M!(") || valor_str.starts_with("V!(") {
                        match eval_m(valor_str, pool) {
                            Ok(v) => Values::U32(v as u32),
                            Err(e) => {
                                println!("Erro ao avaliar expressão: {}", e);
                                return;
                            }
                        }
                    } else {
                        match valor_str.parse::<u32>() {
                            Ok(v) => Values::U32(v),
                            Err(e) => {
                                println!("Erro ao converter valor para U32: {}", e);
                                return;
                            }
                        }
                    }
                }

                Types::I32 => {
                    if valor_str.starts_with("M!(") || valor_str.starts_with("V!(") {
                        match eval_m(valor_str, pool) {
                            Ok(v) => Values::I32(v as i32),
                            Err(e) => {
                                println!("Erro ao avaliar expressão: {}", e);
                                return;
                            }
                        }
                    } else {
                        match valor_str.parse::<i32>() {
                            Ok(v) => Values::I32(v),
                            Err(e) => {
                                println!("Erro ao converter valor para I32: {}", e);
                                return;
                            }
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
                        // Deve vir entre aspas
                        if (valor_str.starts_with('"') && valor_str.ends_with('"'))
                            || (valor_str.starts_with('\'') && valor_str.ends_with('\''))
                        {
                            let conteudo = valor_str[1..valor_str.len() - 1].to_string();
                            Values::String(conteudo)
                        } else {
                            println!("String precisa estar entre aspas: {}", valor_str);
                            return;
                        }
                    }
                }
            };

            create_var(tipo, valor, nome_str, pool);
        }
        "DROP" => {
            for var in pool.iter() {
                if var.nome == resto {
                    remover_var(pool, var as *const Variable);
                    println!("Variável '{}' removida.", resto);
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

            pool[position].valor = match pool[position].tipo {
                Types::Bool => {
                    if novo_valor.starts_with("V!(") {
                        match eval_m(novo_valor, pool) {
                            Ok(v) => match v {
                                1.0 => Values::Bool(true),
                                0.0 => Values::Bool(false),
                                _ => {
                                    println!("Valor inválido para Bool: {}", v);
                                    return;
                                }
                            },
                            Err(e) => {
                                println!("Erro ao avaliar expressão: {}", e);
                                return;
                            }
                        }
                    } else {
                        match novo_valor {
                            "true" => Values::Bool(true),
                            "false" => Values::Bool(false),
                            _ => {
                                println!("Valor inválido para Bool: {}", novo_valor);
                                return;
                            }
                        }
                    }
                }
                Types::U8 => {
                    if novo_valor.starts_with("M!(") || novo_valor.starts_with("V!(") {
                        match eval_m(novo_valor, pool) {
                            Ok(v) => Values::U8(v as u8),
                            Err(e) => {
                                println!("Erro ao avaliar expressão: {}", e);
                                return;
                            }
                        }
                    } else {
                        match novo_valor.parse::<u8>() {
                            Ok(v) => Values::U8(v),
                            Err(e) => {
                                println!("Erro ao converter valor para U8: {}", e);
                                return;
                            }
                        }
                    }
                }

                Types::I8 => {
                    if novo_valor.starts_with("M!(") || novo_valor.starts_with("V!(") {
                        match eval_m(novo_valor, pool) {
                            Ok(v) => Values::I8(v as i8),
                            Err(e) => {
                                println!("Erro ao avaliar expressão: {}", e);
                                return;
                            }
                        }
                    } else {
                        match novo_valor.parse::<i8>() {
                            Ok(v) => Values::I8(v),
                            Err(e) => {
                                println!("Erro ao converter valor para I8: {}", e);
                                return;
                            }
                        }
                    }
                }

                Types::U32 => {
                    if novo_valor.starts_with("M!(") || novo_valor.starts_with("V!(") {
                        match eval_m(novo_valor, pool) {
                            Ok(v) => Values::U32(v as u32),
                            Err(e) => {
                                println!("Erro ao avaliar expressão: {}", e);
                                return;
                            }
                        }
                    } else {
                        match novo_valor.parse::<u32>() {
                            Ok(v) => Values::U32(v),
                            Err(e) => {
                                println!("Erro ao converter valor para U32: {}", e);
                                return;
                            }
                        }
                    }
                }

                Types::I32 => {
                    if novo_valor.starts_with("M!(") || novo_valor.starts_with("V!(") {
                        match eval_m(novo_valor, pool) {
                            Ok(v) => Values::I32(v as i32),
                            Err(e) => {
                                println!("Erro ao avaliar expressão: {}", e);
                                return;
                            }
                        }
                    } else {
                        match novo_valor.parse::<i32>() {
                            Ok(v) => Values::I32(v),
                            Err(e) => {
                                println!("Erro ao converter valor para I32: {}", e);
                                return;
                            }
                        }
                    }
                }
                _ => {
                    println!(
                        "String não pode ser alterada com CHANGE. Use DROP e crie uma nova variável. Não estoure a memória!"
                    );
                    return;
                }
            };
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

            let condicao_resultado =
                if condicao_str.starts_with("B!(") || condicao_str.starts_with("V!(") {
                    match eval_b(condicao_str, pool) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("Erro ao avaliar condição: {}", e);
                            return;
                        }
                    }
                } else {
                    match condicao_str {
                        "true" => true,
                        "false" => false,
                        _ => {
                            println!("Condição inválida para IF: {}", condicao_str);
                            return;
                        }
                    }
                };

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
        _ => {}
    };
}

fn check_condition(condicao_str: &str, pool: &mut Vec<Variable>) -> bool {
    if condicao_str.starts_with("B!(") || condicao_str.starts_with("V!(") {
        match eval_b(condicao_str, pool) {
            Ok(v) => v,
            Err(e) => {
                println!("Erro ao avaliar condição: {}", e);
                return false;
            }
        }
    } else {
        match condicao_str {
            "true" => true,
            "false" => false,
            _ => {
                //println!("Condição inválida para IF: {}", condicao_str);
                return false;
            }
        }
    }
}
unsafe fn read_variables(pool: &mut Vec<Variable>) {
    println!("\n---- Variáveis alocadas ----");

    for var in pool {
        print!("{}: ", var.nome);

        match var.tipo {
            Types::Bool => {
                if let Values::Bool(v) = var.valor {
                    println!("Bool = {}", v);
                }
            }
            Types::U8 => {
                if let Values::U8(v) = var.valor {
                    println!("U8 = {}", v);
                }
            }
            Types::I8 => {
                if let Values::I8(v) = var.valor {
                    println!("I8 = {}", v);
                }
            }
            Types::U32 => {
                if let Values::U32(v) = var.valor {
                    println!("U32 = {}", v);
                }
            }
            Types::I32 => {
                if let Values::I32(v) = var.valor {
                    println!("I32 = {}", v);
                }
            }
            Types::String => {
                if let Values::String(ref v) = var.valor {
                    println!("String = \"{}\"", v);
                }
            }
        }
    }
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

            unsafe {
                interpretar(
                    instrucao,
                    &mut pool,
                    &mut escopos,
                    &mut ignore_next,
                    &mut valid_function,
                    &mut foi_quebrado,
                );
            }
        }

        //read_variables(&mut pool);

        // Não esquecer
        for var in &pool {
            var.destruidor();
        }
        pool.clear();
    }
}
