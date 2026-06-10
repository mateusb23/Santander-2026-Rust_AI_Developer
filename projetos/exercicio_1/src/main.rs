fn main() {
    loop {
        println!("Digite o valor da tabuada que deseja calcular:");
        println!(
            r#"
            Opção 1
            Opção 2
            Opção 3
            Opção 4 - Sair
            "#
        );

        let mut opcao: String = String::new();
        std::io::stdin()
            .read_line(&mut opcao)
            .expect("Falha ao ler a entrada");

        let opcao: u8 = opcao
            .trim()
            .parse()
            .expect("Falha ao converter para número");

        match opcao {
            1 => {
                println!("Você escolheu a opção Um:");
            }
            2 => {
                println!("Você escolheu a opção Dois:");
            }
            3 => {
                println!("Você escolheu a opção Três:");
            }
            4 => {
                println!("Saindo do programa...");
                break;
            }
            _ => {
                println!("Opção inválida!");
            }
        }
    }
}

// fn tabuada(numero: u8) {
//     for i in 1..=10 {
//         println!("{} x {} = {}", numero, i, numero * i);
//     }
// }
