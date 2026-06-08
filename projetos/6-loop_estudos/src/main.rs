fn main() {
    exemplo_loop();
    exemplo_while();
    exemplo_for();
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

fn exemplo_while() {
    let mut copa_do_mundo: i64 = 1998;

    println!("\n----------- COPA DO MUNDO DA FIFA -----------\n");

    while copa_do_mundo <= 2026 {
        println!("Copa do Mundo da FIFA: {}", copa_do_mundo);
        if copa_do_mundo == 2002 {
            println!("(O Brasil foi o CAMPEÃO!)")
        }
        copa_do_mundo += 4;
    }
}

fn exemplo_for() {
    let anos_copa_do_mundo: [i32; 8] = [1998, 2002, 2006, 2010, 2014, 2018, 2022, 2026];

    println!("\n----------- COPA DO MUNDO DA FIFA -----------\n");

    for ano in anos_copa_do_mundo.iter() {
        println!("Copa do Mundo da FIFA: {}", ano);
        if *ano == 2002 {
            println!("(O Brasil foi o CAMPEÃO!)")
        }
    }
}
