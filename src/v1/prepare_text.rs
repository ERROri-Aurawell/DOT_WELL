pub fn processar_linha(linha: &str) -> String {
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
