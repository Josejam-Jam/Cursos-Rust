/*  Ejer 6  Sería interesante disponer de un programa que pida como entrada un número decimal y lo
            muestre redondeado al entero más próximo.
*/

use std::io;
use std::str::FromStr;

fn main() {
    let num;
    let num_entero_proximo: i32;

    println!("\nIntroduce un número decimal");

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

    num = f32::from_str(&entrada.trim()).unwrap();

    num_entero_proximo = (num + 0.5) as i32;
    //num_entero_proximo = get_entero_proximo(num);

    println!("\nEntero más próximo:\t{}\n", num_entero_proximo);
    //println!("\nEntero más proximo:\t{}\n", (num_entero_proximo as i32));

    let salida = get_entero_proximo(num);

    println!("Entero más próximo funcion\t{}\n", salida);

}

/* Función que devuelve el entero más proximo */ 
fn get_entero_proximo(num: f32) -> i32{
    let result = num + 0.5;
    return result as i32;
}
    
