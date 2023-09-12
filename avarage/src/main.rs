fn main() {
    fn media(lista: &[f64]) -> f64 {
        let soma: f64 = lista.iter().sum();
        let tamanho = lista.len() as f64;
        soma / tamanho
    }
}
