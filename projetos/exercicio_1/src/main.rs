fn main() {
    println!("Digite o valor da tabuada que deseja calcular:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let numero: u8 = input.trim().parse().unwrap();

    tabuada(numero);
}

fn tabuada(numero: u8) {
    for i in 1..=10 {
        println!("{} x {} = {}", numero, i, numero * i);
    }
}
