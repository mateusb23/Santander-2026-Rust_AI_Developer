// fn main() {
//     // Variáveis na memória Heap

//     let s1: String = String::from("Olá, mundo!"); // s1 possui a propriedade da string alocada na memória heap
//     let s2: String = s1; // s1 é movida para s2 (BORROWING), e s1 não é mais válida

//     /* print_string(s1); Isso causará um erro de compilação, porque s1 não é mais válida após a atribuição para s2. O Rust tem um sistema de
//     propriedade que impede o uso de variáveis que foram movidas */
//     print_string(&s2); // passando uma referência para s2, que é válida e pode ser usada normalmente.
// }

// fn print_string(s: &String) {
//     println!("O valor da string é: {}, Endereço de memória: {:p}", s, s); // imprimindo o valor e o endereço de memória da string referenciada por s
// }

/* fn main() {
    // Exemplo com String
    let s1: String = String::from("Olá, mundo!"); // s1 possui a propriedade da string alocada na memória heap (variável que pode ser mutável e tem um tamanho dinâmico)
    let s2: String = s1.clone(); // clonando s1 para s2

    println!("String s1: {}, Endereço de memória: {:p}", s1, &s1);
    println!("String s2: {}, Endereço de memória: {:p}", s2, &s2);

    // Exemplo com &str
    let s3: &str = "Olá, mundo!"; // s3 é um &str (slice de string imutável)
    let s4: &str = s3; // s4 é uma referência para s3, não há clonagem envolvida

    println!("String s3: {}, Endereço de memória: {:p}", s3, s3);
    println!("String s4: {}, Endereço de memória: {:p}", s4, s4);
} */

fn main() {
    // Exemplo com String
    let mut s1: String = String::from("Olá, mundo!"); // s1 é um String alocada na memória heap
    s1 += " - teste";

    let s2: String = s1.clone(); // Clonando a String s1 para criar s2

    println!("String s1: {}, Endereço de memória: {:p}", s1, &s1);
    println!("String s2: {}, Endereço de memória: {:p}", s2, &s2);

    // Exemplo com &str
    let s3: &str = "Olá, mundo!"; // s3 é um &str (slice de string imutável)
    // s3 += " - teste"; // Isso causará um erro de compilação, pois &str é imutável

    let s4: &str = s3; // s4 é uma referência para o mesmo &str s3

    println!("String s3: {}, Endereço de memória: {:p}", s3, s3);
    println!("String s4: {}, Endereço de memória: {:p}", s4, s4);
}