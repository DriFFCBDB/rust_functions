// Exercício 9: Função que verifica se um número é primo

fn is_prime(numero: u32) -> bool {
    if numero <= 1 {
        return false;
    } else if numero <= 3 {
        return true;
    } else if numero % 2 == 0 || numero % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= numero {
        if numero % i == 0 || numero % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let numero = 21;
    if is_prime(numero) {
        println!("{} é um número primo.", numero);
    } else {
        println!("{} não é um número primo.", numero);
    }
}
