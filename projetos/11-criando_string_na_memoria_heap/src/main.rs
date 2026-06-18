fn main() {
    // variáveis na memória Heap (variáveis do tipo String no Rust, que são alocadas dinamicamente na memória heap)

    // não é uma variável by copy (variáveis do tipo String não implementam o trait Copy, o que significa que quando atribuídas a outra variável, a propriedade é transferida em vez de ser copiada)
    let s1: String = String::from("Olá, mundo!"); // alocação de uma string na memória heap
    let s2: String = s1; // s1 é movida para s2 (BORROWING), e s1 não é mais válida

    /* Isso causa um erro de compilação, porque s1 não é mais válida após a atribuição para s2. O Rust tem um sistema de propriedade que impede o uso
     de variáveis que foram movidas. Para evitar esse erro, podemos usar uma referência ou clonar a string.
    println!("s1: {}, Endereço de memória: {:p}", s1, &s1); // imprimindo o valor e o endereço de memória de s2 */

    // s2 é válido e pode ser usado normalmente
    println!("s2: {}, Endereço de memória: {:p}", s2, &s2);

    println!("----------------------------------------------------------");

    main_clone();
}

fn main_clone() {
    let s1: String = String::from("Olá, mundo!"); // alocação de uma string na memória heap
    let s2: String = s1.clone(); // clonando s1 para s2, criando uma cópia independente da string

    println!("s1: {}, Endereço de memória: {:p}", s1, &s1); // imprimindo o valor e o endereço de memória de s1
    println!("s2: {}, Endereço de memória: {:p}", s2, &s2); // imprimindo o valor e o endereço de memória de s2
}
