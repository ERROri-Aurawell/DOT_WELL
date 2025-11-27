//Como criar variável:
//VAR:Tipo:Nome = Valor;
//Tipos: Bool, U8, I8, U32, I32, String
//Também aceito:
//VAR:Tipo:Nome=Macro!(expressão);
//Tipos de macro: M!(...) para números, V!(...) para variáveis, B!(...) para booleanos


//Como criar uma função:
//FN:nome_da_função{código_da_função}FN;

//Como deletar variável:
//DROP:nome_da_variável;

//Como alterar valor da variável:
//CHANGE:nome_da_variável:novo_valor;

//Aceito também:
//CHANGE:nome_da_variável=Macro!(expressão);

//Como chamar função:
//EXECUTE:nome_da_função;

// Como um IF funciona:
// IF:condição:função;

// Aceita também:
// IF:Macro!(expressão):função;

MACROS:

//Exemplos de macros:
//M!() - Matemática (retorna número)
//V!() - Variável (retorna valor da variável)
//B!() - Booleano (retorna valor booleano)
//Exemplos:
M!(5 + 10 * 2) // Retorna 25
V!(variavel_1) // Retorna o valor da variável 'variavel_1'
B!(variavel_2 == 100) // Retorna true ou false
//Exemplo completo:
FN:main{
    U32 : variavel_1 = M!(5 + 10 * 2);
    Bool : variavel_2 = B!(variavel_1 == 25);
    String : mensagem = "O valor de variavel_1 é: ";
    // Imprimir mensagem (ainda não implementado, mas futuramente terá uma função de print)
    //PRINT: C!(V!(mensagem),V!(variavel_1));
}FN;