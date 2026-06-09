fn main() {
    println!("Digite o valor da tabuada que deseja calcular:");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada");

    let numero: u8 = input
        .trim()
        .parse()
        .expect("Falha ao converter para número");

    tabuada(numero);
}

fn tabuada(numero: u8) {
    for i in 1..=10 {
        println!("{} x {} = {}", numero, i, numero * i);
    }
}
