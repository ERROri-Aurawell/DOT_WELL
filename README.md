# Projeto DOT_WELL (V2.1)

DOT_WELL é uma linguagem de programação interpretada, inspirada em linguagens como Rust, C e JavaScript. Ela é projetada para ser explícita e fortemente tipada.

## Características Principais
- **Tipagem Estática Forte**: Todas as variáveis devem ter seus tipos definidos explicitamente no momento da criação.
- **Sintaxe Simplificada**: A V2 introduziu uma forma mais limpa de declarar variáveis, e a V2.1 aprimora a modificação de valores com operadores de atribuição e incremento.
- **Controle de Fluxo**: Suporte para condicionais (`IF`) e laços de repetição (`WHILE`).
- **Funções**: Permite a definição e chamada de blocos de código reutilizáveis.
- **Macros para Expressões**: Avaliação de expressões matemáticas, booleanas e de acesso a variáveis através de macros.

## Documentação
A sintaxe da V1 (usando `VAR:`, `CHANGE:var:valor`) ainda é parcialmente suportada, mas a nova sintaxe é preferível.

### Variáveis
A declaração de variáveis é feita informando o tipo, nome e valor.

**Sintaxe:**
`Tipo:Nome = Valor;`

**Tipos Suportados:**
- `String`
- `Bool`
- `U8`, `I8`
- `U32`, `I32`

**Exemplos:**
```
// Declaração de variáveis de diferentes tipos
U32:numero = 100;
Bool:verdadeiro = true;
String:mensagem = "Olá, mundo!";
```

#### Modificação de Variáveis
A V2.1 introduz operadores de atribuição e incremento/decremento para uma manipulação mais fluida das variáveis.

**Sintaxe:**
`nome_da_variavel operador valor;`

**Operadores Suportados:**
- Atribuição simples: `=`
- Atribuição com operação: `+=`, `-=`, `*=`, `/=`
- Incremento/Decremento: `++`, `--`

**Exemplos:**
```
numero = 200;
numero += 50; // numero agora é 250
numero--; // numero agora é 249
DROP:numero;
```

### Funções
Funções são blocos de código que podem ser definidos e executados posteriormente.

**Sintaxe:**
`FN:nome_da_funcao{ ...código... }FN;`

**Executando uma função:**
`EXECUTE:nome_da_funcao;`

**Exemplo:**
```
FN:saudacao{
    PRINT:"Executando a função de saudação!";
}FN;

EXECUTE:saudacao;
```

### Controle de Fluxo

#### IF
Executa uma função se uma condição for verdadeira.

**Sintaxe:**
`IF:condicao:nome_da_funcao;`

**Exemplo:**
```
U8:idade = 18;

FN:maior_de_idade{
    PRINT:"É maior de idade.";
}FN;

IF:B!(V!(idade) >= 18):maior_de_idade;
```

#### WHILE
Executa uma função repetidamente enquanto uma condição for verdadeira. A palavra-chave `BREAK` pode ser usada dentro de um `IF` para sair do laço.

**Sintaxe:**
`WHILE:condicao:nome_da_funcao;`

**Exemplo:**
```
U8:contador = 0;

FN:incrementa{
    CHANGE:contador:M!(V!(contador) + 1);
    PRINT:T!("Contador: " V!(contador));
    IF:B!(V!(contador) == 5):BREAK;
}FN;

WHILE:B!(V!(contador) < 10):incrementa;
```

### Macros
Macros são usadas para avaliar expressões complexas.

- `M!(...)`: Para expressões **M**atemáticas.
- `B!(...)`: Para expressões **B**ooleanas.
- `V!(...)`: Para obter o **V**alor de uma variável.
- `T!(...)`: Para concatenar e transformar valores em **T**exto (String).

**Exemplos:**
```
U32:resultado = M!( (10 + 5) * 2 ); // resultado = 30
Bool:comparacao = B!(V!(resultado) > 20); // comparacao = true
String:texto_final = T!( "O resultado é " V!(resultado) " e a comparação é " V!(comparacao) );

PRINT:V!(texto_final);
```

### Entrada e Saída

#### PRINT
Imprime valores no console.

**Sintaxe:**
`PRINT:valor;`

**Exemplo:**
```
String:nome = "DOT_WELL";
PRINT:T!("Olá, " V!(nome) "!"); // Imprime "Olá, DOT_WELL!"
```

#### INSERT
Lê uma entrada do usuário no console e a atribui a uma variável `String`.

**Exemplo:**
```
PRINT:"Qual é o seu nome?";
String:nome_usuario = INSERT;
PRINT:T!("Bem-vindo, " V!(nome_usuario) "!");
```
