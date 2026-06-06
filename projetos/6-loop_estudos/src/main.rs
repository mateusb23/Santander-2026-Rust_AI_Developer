fn main() {
    exemplo_loop();
}

fn exemplo_loop() {
    let mut contador: i32 = 0;

    loop {
        println!("Contador: {}", contador);
        contador += 1;

        if contador >= 5 {
            break;
        }
    }
}
