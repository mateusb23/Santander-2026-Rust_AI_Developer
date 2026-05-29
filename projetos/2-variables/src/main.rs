/* fn main() {
    let x: i32 = 5; // x é imutável por padrão
    println!("O valor de x é: {}", x);
    x = 6; // Isso causará um erro de compilação, pois x é imutável por padrão
    println!("O valor de x é: {}", x);
} */

fn main() {
    let mut x: i32 = 5; // Agora x é mutável
    println!("O valor de x é: {}", x);
    x = 6; // Isso agora é permitido
    println!("O valor de x é: {}", x);
    let x: i32 = 7; // Isso é uma nova variável x, que é imutável devido ao let sem mut
    println!("O valor de x é: {}", x);
}