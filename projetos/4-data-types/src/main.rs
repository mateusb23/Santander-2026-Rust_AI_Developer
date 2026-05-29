fn main() {
    // Em Rust, existem vários tipos de dados primitivos, incluindo:
    // Inteiros (i8, i16, i32, i64, i128, u8, u16, u32, u64, u128)
    // Ponto flutuante (f32, f64)
    // isize e usize (tipos de inteiro dependentes do tamanho da arquitetura)
    // Booleanos (bool)
    // Caracteres (char)
    // Strings (String e &str)
    // Tuplas (tuples)
    // Arrays (arrays)
    
    //// Exemplo de uso de todos os tipos de dados:

    let inteiro_8: i8 = 10; // Inteiro de 8 bits
    let inteiro_16: i16 = 100; // Inteiro de 16 bits
    let inteiro_32: i32 = 1000; // Inteiro de 32 bits
    let inteiro_64: i64 = 10000000000; // Inteiro de 64 bits
    let inteiro_128: i128 = 100000000000000000000; // Inteiro de 128 bits
    let inteiro_u8: u8 = 255; // Inteiro sem sinal de 8 bits
    let inteiro_u16: u16 = 65535; // Inteiro sem sinal de 16 bits
    let inteiro_u32: u32 = 4294967295; // Inteiro sem sinal de 32 bits
    let inteiro_u64: u64 = 18446744073709551615; // Inteiro sem sinal de 64 bits
    let inteiro_u128: u128 = 340282366920938463463374607431768211455; // Inteiro sem sinal de 128 bits
    let isize_value: isize = 1000; // Inteiro dependente da arquitetura
    let usize_value: usize = 2000; // Inteiro sem sinal dependente da arquitetura
    let ponto_flutuante_32: f32 = 3.14; // Ponto flutuante de 32 bits
    let ponto_flutuante_64: f64 = 3.14; // Ponto flutuante de 64 bits
    let booleano: bool = true; // Booleano
    let caractere: char = 'R'; // Caractere
    let string: &str = "Olá, Rust!"; // String slice --> é uma fatia de string, que é imutável e tem um tamanho fixo
    let string_owned: String = "Olá, Rust!".into(); // String owned --> é uma string que possui seus próprios dados e pode ser mutável
    let mut x: String = String::from("Olá, Rust!"); // String mutável
    x.push_str(" Bem-vindo ao Rust!"); // Modificando a string mutável
    let tupla: (i32, f64, bool) = (42, 3.14, true); // Tupla
    let array: [i32; 3] = [1, 2, 3]; // Array
    let array_imutavel: [i32; 4] = [4, 5, 6, 56]; // Array imutável
    let mut array_mutavel: [i32; 3] = [87, 9, 49]; // Array mutável
    array_mutavel[0] = 21;  // Modificando o primeiro elemento do array mutável



    println!("Inteiro 8-bit: {}", inteiro_8);
    println!("Inteiro 16-bit: {}", inteiro_16);
    println!("Inteiro 32-bit: {}", inteiro_32);
    println!("Inteiro 64-bit: {}", inteiro_64);
    println!("Inteiro 128-bit: {}", inteiro_128);
    println!("Inteiro sem sinal 8-bit: {}", inteiro_u8);
    println!("Inteiro sem sinal 16-bit: {}", inteiro_u16);
    println!("Inteiro sem sinal 32-bit: {}", inteiro_u32);
    println!("Inteiro sem sinal 64-bit: {}", inteiro_u64);
    println!("Inteiro sem sinal 128-bit: {}", inteiro_u128);
    println!("isize: {}", isize_value);
    println!("usize: {}", usize_value);
    println!("Ponto flutuante 32-bit: {}", ponto_flutuante_32);
    println!("Ponto flutuante 64-bit: {}", ponto_flutuante_64);
    println!("Booleano: {}", booleano);
    println!("Caractere: {}", caractere);
    println!("String slice: {}", string);
    println!("String owned: {}", string_owned);
    println!("String mutável: {}", x);
    println!("Tupla: ({}, {}, {})", tupla.0, tupla.1, tupla.2);
    println!("Array: [{}, {}, {}]", array[0], array[1], array[2]);
    println!("Array imutável: [{}, {}, {}]", array_imutavel[0], array_imutavel[1], array_imutavel[2]);
    println!("Array mutável: [{}, {}, {}]", array_mutavel[0], array_mutavel[1], array_mutavel[2]);  
}
