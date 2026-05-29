fn main() {
    // Comentários em Rust podem ser assim
    /* Nesse programinha simples iremos mostrar como calcular a idade de uma pessoa chamada Fabiana */

    let nome: &str = "Fabiana";

    let ano_nascimento: i16 = 2002;
    let mes_nascimento: i8 = 12; 
    let ano_atual: i16 = 2024;

    let idade: i16 = ano_atual - ano_nascimento;

    println!("A idade da pessoa ({}) calculada para o ano de {} é de {} anos.", nome, ano_atual, idade);
}
