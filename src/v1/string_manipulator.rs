pub fn separate_quoted_and_unquoted(input: &str) -> (String, String) {
    let mut quoted: String = "\"".to_string();
    let mut unquoted: String = "".to_string();

    let characters: Vec<char> = input.chars().collect();

    let init_char: char = characters[0];
    let mut string_end: bool = false;

    let mut counter = 0;
    for i in characters {
        if counter == 0 {
            counter += 1;
            continue;
        };
        let character = i.to_string();
        //println!("{character}");

        if i == init_char {
            //println!("CARACTERE DE FINALIZAÇÂO");
            quoted = quoted + init_char.to_string().as_str();
            string_end = true;
        }

        if !string_end {
            quoted = quoted + character.as_str();
        } else {
            if character == ":".to_string()
                || character == " ".to_string()
                || character == init_char.to_string()
            {
                continue;
            }

            unquoted = unquoted + character.as_str();
        }
    }

    let unquoted = unquoted.trim().to_string();

    (quoted, unquoted)
}

pub fn separate_string_functions(input: String) -> Vec<String> {
    let valores: String = input.chars().skip(1).collect();
    let valores: String = valores.chars().rev().skip(2).collect();
    let valores: String = valores.chars().rev().collect();

    let functions: Vec<String> = valores.split(",").map(|s| s.to_string()).collect();

    functions
}