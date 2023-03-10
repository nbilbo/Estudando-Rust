use std::io;

fn main() {
    let mut nome: String = String::new();
    println!("Ola, informe seu nome de usuario");

    io::stdin()
        .read_line(&mut nome)
        .expect("Erro de leitura");

    println!("Bem vindo {nome}");
}
