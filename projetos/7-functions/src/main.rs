fn main() {
    mostrar_oi_na_tela();
    // MOSTRANDO COMO TODA FUNÇÃO RETORNA UM VALOR, MESMO QUE SEJA O TIPO UNITÁRIO `()`
    let valor_retorno: () = mostrar_oi_na_tela();
    println!("Valor de retorno da função: {:?}", valor_retorno);

    let valor_retorno_com_retorno: String = funcao_com_retorno();
    println!(
        "Valor de retorno da função com retorno: {}",
        valor_retorno_com_retorno
    );
}

fn mostrar_oi_na_tela() {
    // no Rust toda função retorna um valor, mesmo que seja o tipo unitário `()`, que é representado por parênteses vazios.
    println!("Oiiiiii!");
}

fn funcao_com_retorno() -> String {
    return "Essa função retorna uma String".to_string();
}
