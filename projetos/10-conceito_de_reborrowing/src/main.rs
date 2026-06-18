// fn main() {
//     // memória stack (variáveis do tipo copy no Rust, como tipos primitivos: i32, f64, bool, etc.)
//     let x: i32 = 4;

//     imprime_valor(&x); // passando uma referência imutável para a função imprime_valor, que não pode modificar o valor de x
// }

// fn imprime_valor(valor: &i32) {
//     // valor += 1; // nãp pode porque tenho imutabilidade nas referências e não posso modificar o valor apontado por uma referência imutável
//     println!("Valor {}, Endereço de memória: {:p}", valor, valor);
// }

fn main() {
    let mut x: i32 = 4;

    imprime_valor(&mut x); // passando uma referência mutável para a função imprime_valor, que pode modificar o valor de x
    println!(
        "Valor de x após a chamada da função: {}, Endereço de memória: {:p}",
        x, &x
    ); // // o endereço de memória impresso aqui vai ser o do valor original de x, e não o endereço de memória do valor temporário criado pelo reborrowing dentro da função imprime_valor.

    imprime_valor(&mut x);
    println!(
        "Valor de x após a chamada da função: {}, Endereço de memória: {:p}",
        x, &x
    );
}

fn imprime_valor(valor: &mut i32) {
    *valor += 1; // modificando o valor referenciado por valor utilizando um reborrowing.
    // O compilador pode mover a variável temporariamente para uma localização diferente na memória durante a referência mutável.
    // O objetivo é evitar possíveis problemas de aliaing e garantir a segurança das referÊncias mutáveis.
    println!("Valor {}, Endereço de memória: {:p}", valor, &valor); // o endereço de memória impresso aqui vai ser o do valor temporário criado pelo reborrowing, e não o endereço de memória original de x.
}
