fn main() {
    mostra_na_tela(1);
}

fn mostra_na_tela(i: i32) {
    if i > 10 {
        return; // condição de parada para evitar recursão infinita
    }

    println!("O valor de i é: {}", i);
    mostra_na_tela(i + 1);
}
