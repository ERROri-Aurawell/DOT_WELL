use crate::v1::var_maker::Funcao;

pub struct function_props {
    pub em_construcao: bool,
    pub ignorar_chaves: u16,
}

pub fn montar_funcao(linha: &str, f_atual: &mut function_props, funcoes: &mut Vec<Funcao>){
    let posicao = funcoes.len() - 1;
    let mut entr_codigo = "".to_string();

    //println!("----------\nChecando linha: {} \n", linha);
    for c in linha.chars() {
        if c == '{' {
            f_atual.ignorar_chaves += 1;
            continue;
        }

        if c == '}' {
            if f_atual.ignorar_chaves == 0 {
                println!("Erro: caractere inválido → }}");
                f_atual.em_construcao = false;
                break;
            }
            f_atual.ignorar_chaves -= 1;
            //println!("Aqui tem um }}");
        }

        if f_atual.ignorar_chaves == 0 {
            f_atual.em_construcao = false;
            break;
        }
        entr_codigo += c.to_string().as_str();
    }

    //println!("Esperar {} chave/s", f_atual.ignorar_chaves);
    if f_atual.ignorar_chaves == 0 {
        //println!("Aqui acaba a construção");
        f_atual.em_construcao = false;
    } else {
        let texto = format!("{};", &entr_codigo);
        funcoes[posicao].codigo.push_str(&texto);
    }
}
