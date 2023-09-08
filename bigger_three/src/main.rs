// Exercício 6: Função que retorna o maior de três números
fn maior_de_tres(a: i32, b: i32, c: i32) -> i32 {
    if a >= b && a >= c {
        a
    } else if b >= a && b >= c {
        b
    } else {
        c
    }
}

