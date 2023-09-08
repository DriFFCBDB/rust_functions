// Exercício 3: Função que verifica se um número é par
fn par(numero: i32) -> bool {
    numero % 2 == 0
}

fn main() {
    let numero = 10; 

    let par_verifica = par(numero);
    println!("Verification : {}", par_verifica);

    if par_verifica == true {
        println!("Este numero é par.");
    } else {
        println!("Este numero não é par.");
    }
}


