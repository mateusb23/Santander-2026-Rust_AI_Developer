const TIPO_DE_DADO:i8 = 2;
static UMA_VARIAVEL_STATICA: i8 = 3;

fn main() {
    println!("Constante: {}", TIPO_DE_DADO);
    println!("Estática: {}", UMA_VARIAVEL_STATICA);
    imprime();
}

fn imprime() {
    println!("Constante: {}", TIPO_DE_DADO);
    println!("Estática: {}", UMA_VARIAVEL_STATICA);
}