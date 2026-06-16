// O que é Ownership e Borrowing no Rust?
// Ownership é um sistema de gerenciamento de memória que o Rust utiliza para garantir a segurança e eficiência do código. Ele é baseado em três regras principais:
// 1. Cada valor no Rust tem um proprietário (owner).
// 2. Só pode haver um proprietário por vez.
// 3. Quando o proprietário sai de escopo, o valor é descartado (deallocated).
// Borrowing é o processo de emprestar um valor para outra parte do código sem transferir a propriedade. Existem dois tipos de borrowing: borrowing imutável (immutable borrowing) e
// borrowing mutável (mutable borrowing). O borrowing imutável permite que várias partes do código leiam um valor ao mesmo tempo, enquanto o borrowing mutável permite que apenas uma parte do código modifique um valor, garantindo a segurança de acesso à memória.
// O Rust utiliza o sistema de ownership e borrowing para evitar problemas comuns de gerenciamento de memória, como vazamentos de memória (memory leaks) e condições de corrida

////////////////////// EXEMPLOS ///////////////////////////

// Exemplo 1

// fn main() {
//     // memória stack (variáveis do tipo copy no Rust, como tipos primitivos: i32, f64, bool, etc.)
//     let x: i32 = 4;
//     let y: i32 = x; // aqui ocorre uma cópia do valor de x para y, pois i32 é um tipo copy

//     println!("O valor de x é: {} - Referência de memória: {:p}", x, &x);
//     println!("O valor de y é: {} - Referência de memória: {:p}", y, &y);
// }

// Exemplo 2

fn main() {
    // memória stack (variáveis do tipo copy no Rust, como tipos primitivos: i32, f64, bool, etc.)
    let x: i32 = 4; // owner
    let y: &i32 = &x; // referência de dados (y aponta para o mesmo local de x, sem copiar o valor)

    println!("O valor de x é: {} - Referência de memória: {:p}", x, &x);
    println!("O valor de y é: {} - Referência de memória: {:p}", y, y);
}
