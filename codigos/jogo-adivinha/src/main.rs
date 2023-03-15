// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// dependencias.
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // imprimindo o cabecalho.
    println!("🕹 Jogo de adivinhacao");
    
    // gerando um valor aleatorio.
    let valor: u8 = rand::thread_rng().gen_range(1..=100);
    println!("[DEBUG] O valor secreto e: {}", valor);
    
    loop { // loop, encerra se o jogador acertar.
        println!("Digite seu palpite (1 a 100):");
        // primeiro recebemos o palpite do usuario.
        let mut palpite = String::new();
        io::stdin()
            .read_line(&mut palpite)
            .expect("Erro ao ler a linha");
        
        // convetemos para um inteiro.
        let palpite: u8 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Voce palpitou o valor {}", palpite);
        
        // verificamos.
        match palpite.cmp(&valor) {
            Ordering::Less => println!("Muito baixo"),
            Ordering::Greater => println!("Muito alto"),
            Ordering::Equal => {
                println!("Voce acertou");
                break;
            }
        }
    } // fim do loop.
}
