// um pouco sobre os tipos primitivos.

fn main() {
    let idade: u8 = 10; // inteiro sem sinal de 8 bits.
    let peso = 10_u8; // ubteuri sen subak de 8 bits.
    let milhao = 1_000_000; // melhorando a legibilidade.
    let hexadecimal = 0xf; // hexadecimal.
    let media: f64 = 42.1; // ponto flutuante.
    let verdadeiro: bool = true; // boleano.
    let falso: bool = false; // boleano.
    let emoji: char = '😀'; // caractere.
    let numeros: (u8, u8, char) = (1, 2, '3'); // tupla.
    let matriz: [char; 5] = ['a', 'b', 'c', 'd', 'e']; // matriz(array).

    println!("Idade: {}", idade);
    println!("Peso: {}", peso);
    println!("Milhao: {}", milhao);
    println!("Hexadecimal: {}", hexadecimal);
    println!("Média: {}", media);
    println!("Verdadeiro: {}, Falso: {}", verdadeiro, falso);
    println!("Emoji: {}", emoji);
    println!("Tupla: {:?}", numeros);
    println!("Tupla.1: {}", numeros.1);
    println!("Matriz: {:?}", matriz);
    println!("Matriz[0]: {}", matriz[0]);
    println!("&matriz[0..3]: {:?}", &matriz[0..3]);
}
