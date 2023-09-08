// Exercício 2: Função que retorna a soma de dois números

fn main() {
   
   let resultado_soma = add_numbers(2, 4);
   println!("Soma de dois números é: {}",resultado_soma)
    
}

fn add_numbers(a: i8, b: i8) -> i8 {
    a + b
}