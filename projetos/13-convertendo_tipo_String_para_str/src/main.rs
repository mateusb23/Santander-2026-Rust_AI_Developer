fn main() {
    // Exemplo com String
    let mut s1: String = String::from("Olá, mundo!"); // s1 é um String alocada na memória heap
    s1 += " - teste";

    let s2: String = s1.clone(); // Clonando a String s1 para criar s2

    println!("String s1: {}, Endereço de memória: {:p}", s1, &s1);
    println!("String s2: {}, Endereço de memória: {:p}", s2, &s2);

    // Exemplo com &str
    let s3: &str = "Olá, mundo!"; // s3 é um &str (slice de string imutável)
    
    let s4: String = format!("{} - teste", s3); // Criando um novo &str concatenando s3 com " - teste" e armazenando em s4 como String

    println!("String s3: {}, Endereço de memória: {:p}", s3, s3);
    println!("String s4: {}, Endereço de memória: {:p}", s4, &s4);
}

