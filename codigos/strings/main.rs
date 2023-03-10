// trabalhando com strings.
use std::iter::FromIterator;

fn main() {
    let l0: char = 'f';
    let l1: char = 'e';
    let l2: char = 'l';
    let l3: char = 'i';
    let l4: char = 'p';
    let l5: char = 'e';
    println!("Nome: {l0}{l1}{l2}{l3}{l4}{l5}");

    let sobrenome: &str = "Morais"; // string slice, string reference.
    println!("Sobrenome: {sobrenome}");

    let mut cpf: String = String::new(); // String.
    cpf.push('0');
    cpf.push('0');
    cpf.push('0');
    cpf.push('-');
    cpf.push('0');
    cpf.push('0');
    println!("CPF: {cpf}");

    let mut rg: String = String::new();
    rg.push_str("AA-AAA-AAA");
    println!("RG: {rg}");

    let endereco = ['l', 'i', 'm', 'o', 'e', 'i', 'r', 'o'];
    let endereco_string = String::from_iter(endereco);
    println!("Endereco: {endereco_string}");
}
