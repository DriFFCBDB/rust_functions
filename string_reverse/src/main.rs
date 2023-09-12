fn main() {
    
    let string_reverse = "String Normal e invertida";
     
    println!("A string original é: {}", string_reverse);
    
    let invertida: String = string_reverse.chars().rev().collect();

    println!("A string invertida é: {}", invertida);
  }