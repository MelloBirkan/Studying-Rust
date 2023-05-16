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
  let character: char = 'A';
  // convert char type to u8 integer type
  let integer = character as u8;
  // only u8 integer data type can be converted into char
  let integer: u8 = 65;
  // convert integer to char using the as keyword
  let character = integer as char;
  
  // Operator
  println!("\nOperator");
  let a = 20;
  let b = 2;
  // add two variables using + operator
  let x = a + b;
  println!("{} + {} = {}", a, b, x);
  // subtract two variables using - operator
  let y = a - b;
  println!("{} - {} = {}", a, b, y);
  // multiply two variables using * operator
  let z = a * b;
  println!("{} * {} = {}", a, b, z);
  let modulo = a % b;
  println!("{modulo}");

  // if-else
  let condition = a > b;
  println!("{condition}");
  if condition {
    println!("A condicao é verdadeira")
} else {
    println!("A condição é falsa")
}
  let number = 2;
  if number > 4 {
    println!("O numero é maior que 4");
  } else if number > 0 && number <= 2 {
    println!("O numero é maior que 0");
  } else if number > 0 && number > 2 {
    println!("O numero é maior que 2");
  } else {
    println!(" o número é menor que zero");
  }
  // Nested if-else
  let number = -2;

  if number < 0 {
    if number == -2 {
      println!("O número é -2");
    } else {
      println!("O número não é -2");
    }
  } else {
      println!("O número é maior que zero");
  }

  // Loops
  println!("\nLoops");
  loop {
    println!("esse é um loop infinito");
    break;
  }
  let mut i = 0;
  while i < 3 {
    print!("{i}, ");
    i +=1;
  }
  println!("Nested loop");
  // outer loop counter
    let mut i = 1;
    // outer loop
    while i <= 5 {
        // inner loop counter
        let mut j = 1;
        // inner loop
        while j <= 5 {
            print!("*");
            // increase inner loop counter
            j += 1;
        }
        println!("");
        // increase outer loop counter
        i += 1;
    }

  //for-in
  for i in 1..=6{
    print!("{i}");
  }
  println!("");
  for i in 1..3 {
    print!("{i}");
  }
  let mut n = 0;
  while n < 3 {
    if n == 1 {
      continue
  }
    print!("{n}, ");
    n += 1;
  }

 

  
}