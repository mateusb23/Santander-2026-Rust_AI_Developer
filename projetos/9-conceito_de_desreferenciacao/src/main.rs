fn main() {
    let x: i32 = 4; // Owner (proprietário) do valor 4 é a variável x. O valor 4 é armazenado na memória stack, e x é o responsável por gerenciar esse valor.
    let y: &i32 = &x; // y é uma referência para x, ou seja, y aponta para o mesmo local de memória onde x está armazenado, sem criar uma cópia do valor. Isso é um exemplo de desreferenciação, onde y é uma referência que aponta para o valor de x.

    println!("O valor de x é {}", x);
    println!("O valor de y é {}", y);

    // Imprimindo os endereços de memória de x e y
    println!("Endereço de memória de x: {:p}", &x); // {:p} imprime o endereço de memória da variável x
    println!("Endereço de memória de y: {:p}", y); // y já é uma referência, então {:p} imprime o endereço de memória para o qual y aponta, que é o mesmo endereço de x.

    let t: &i32 = y; // cria outra referência para o dono x
    println!("Endereço de memória de t: {:p}", t);

    let w: i32 = *y; // desreferenciação com copy para w, w é uma cópia do valor de x, pois i32 é um tipo copy
    println!("O valor de w é {}", w);
    println!("Endereço de memória de w: {:p}", &w);
}
