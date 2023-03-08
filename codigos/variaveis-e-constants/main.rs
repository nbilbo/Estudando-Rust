// minhas constantes.
const _SEGUNDOS_EM_MINUTOS: u32 = 60;
const SEGUNDOS_EM_HORAS: u32 = 3600;


fn main() { // escopo.
    // essa variavel não pode ter seu valor redefinido.
    let total = 30;
    println!("Trabalhou {} horas.", total);
    
    // agora essa variavel pode ter seu valor redefinido.
    let mut dias = 5;
    println!("Trabalhou {} dias.", dias);

    dias = 10;
    println!("Trabalhou {} dias.", dias);

    // um outro escopo interno 
    {
        let total = 40;
        println!("Trabalhou {} horas.", total);
    }

    println!("Total de segundos trabalhados:{}", total*SEGUNDOS_EM_HORAS);
} // fim do escopo.

