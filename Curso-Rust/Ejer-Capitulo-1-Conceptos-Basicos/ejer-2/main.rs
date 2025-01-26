/*  Ejer 2  Pedir al usuario su edad y mostrar la que tendrá el próximo año

*/

use std::io;
use std::str::FromStr;

fn main() {
    let mut edad;

    println!("\nIntroduce tú edad");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
    //let mut edad = u32::from_str(&entrada.trim()).unwrap(); 
    edad = u32::from_str(&entrada.trim()).unwrap(); 

    //edad = edad + 1;
    edad += 1; 

    println!("El próximo año tendrás {}\n", edad);
    println!("\nEl próximo año tendrás {} años\n", edad);
    

}
