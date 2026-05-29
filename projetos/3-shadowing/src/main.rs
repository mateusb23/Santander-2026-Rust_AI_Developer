fn main() {
    let x: i32 = 5; // x é imutável por padrão
    println!("O valor de x e sua memória: {}, {:p}", x, &x);

    let x: i32 = x + 1; // Isso é uma nova variável x, que é imutável devido ao let sem mut
    println!("O valor de x e sua memória: {}, {:p}", x, &x);


    let x: i32 = x * 2; // Isso é outra nova variável x, que é imutável devido ao let sem mut
    println!("O valor de x e sua memória: {}, {:p}", x, &x);

    println!("O valor de x é: {}", x);
}
