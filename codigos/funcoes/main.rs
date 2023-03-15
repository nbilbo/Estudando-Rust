fn say_hello(name: &str) {
    println!("Hello, {name}.");
}

fn add_numbers(a: i32, b: i32) ->i32 {
    // um early-return, basicamente para garantir
    // a "parada" da funcao.
    if a == 0 {
        return b;
    }

    // em rust, a ultima expressao sem ponto-e-virgula 
    // é o retorno da funcao.
    // nesse caso, eu nao utilizei a palavra return.
    a+b
}

fn main() {
    say_hello("Amanda");
    let rest = add_numbers(8, 8);
    println!("8+8={rest}");

    let string_values: &str = "10 20 30 40 50 60 70";
    let int_values: Vec<i32> = string_values
        .split(' ')
        .map(|v| v.parse::<i32>().unwrap())
        .map(|x| x*2)
        .collect();
    println!("{:?}", int_values);
}
