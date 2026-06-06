fn main() {
    // Comentários em Rust podem ser assim
    /* Nesse programinha simples iremos mostrar como calcular a idade de uma pessoa chamada Fabiana */

    let nome: &str = "Fabiana";

    let ano_nascimento: i16 = 2002;
    let mes_nascimento: i8 = 12;
    let ano_atual: i16 = 2026;

    let idade: i16 = ano_atual - ano_nascimento;

    let verifica_seculo = if ano_nascimento < 2000 {
        "Nascido(a) no século XX"
    } else {
        "Nascido(a) no século XXI"
    };

    println!(
        "{} em {} {}, {} tem {} anos e {}",
        verifica_seculo,
        ano_nascimento,
        verifica_estado("PE"),
        nome,
        idade,
        verifica_mes(mes_nascimento)
    );
}

fn verifica_mes(mes_nascimento: i8) -> String {
    if mes_nascimento == 12 {
        return "ainda vai fezer aniversário este ano.".to_string();
    } else {
        return "já não fez aniversário este ano.".to_string();
    }
}

fn verifica_estado(uf: &str) -> String {
    match uf {
        "SP" => "no estado de São Paulo".to_string(),
        "RJ" => "no estado do Rio de Janeiro".to_string(),
        "PE" => "no estado de Pernambuco".to_string(),
        _ => "não mora em São Paulo nem no Rio de Janeiro nem em Pernambuco".to_string(),
    }
}
