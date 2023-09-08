// Exercício 4: Função que calcula o fatorial de um número

fn fatorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * fatorial(n - 1)
    }
}