/*  Ejer 1  Diseñar un programa que pida un número al usuario -por teclado- 
            y a continuación lo muestre.
*/

use std::io;

fn main() {
    
        println!("\nIntroduce un número por teclado!");
        //let mut intro_teclado = String::new();
        let mut intro_teclado : String = String::new();
        io::stdin().read_line(&mut intro_teclado).expect("Error en la lectura");
        //io::stdin().read_line(&mut intro_teclado).unwrap();
        let salida = intro_teclado.trim().to_string() +"\n";

        println!("\nEl Número introducido es {}", salida);
        
}

