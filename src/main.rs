fn main() {
  // Rust tem 2 tipos de prints:
  // println! e print!
  println!("Tipos de print");
  print!("print: Esse é um print sem quebra linha ");
  println!("println: Esse ja tem quebra de linha");
  println!("println: Como pode ver");

  // Print com variaveis:
  println!("\nPrint com Variaveis");
  let age = 23;
  let name = ");Marcello";
  println!("O idade é: {}", age); // {} é um placeholder para a variavel
  println!("Meu nome é: {1}, e minha idade é: {0}", age, name);
  println!("Meu nome é: {name}, e tenho {age}");

  // Variaveis em Rust, na convenção damos o nome _case_sensitive_
  println!("\nVariaveis em Rust");
  let hair: &str = "Loiro"; 
  let mut salary: f64 = 25_600.50; // f64 é o tipo da variavel e (mut) deixa ela mutavel
  salary = 28_000.50; // Pode ser alterado o tipo da variavel

  // Constantes, na convenção do Rust usamos letras maiusculas para constantes
  println!("\nConstantes");
  const ID: i32 = 123;

  // Casting
  println!("\nCasting");
  // create a floating-point variable
  let decimal: f64 = 54.321;
  // convert floating point type to integer type
  let integer = decimal as u16;

  
}