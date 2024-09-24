/*  Ejer 2  Pedir dos números enteros y decir si son iguales o no   */

use std::io;
use std::str::FromStr;

fn main() {
    let num1: i32;
    let num2: i32;
    let son_iguales;

    println!("\nIntroduce un número entero");
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
    num1 = i32::from_str(&entrada.trim()).unwrap();

    println!("Introduce un segundo número Enetero");
    let mut entrada2 = String::new();

    io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");
    num2 = i32::from_str(&entrada2.trim()).unwrap();

        if num1 == num2 {
            son_iguales = "Son Iguales";
        } else {
            son_iguales = "No son Iguales";
        }

    println!("\nLos Números \t{} y {}\t{}\n", num1, num2, son_iguales);

}
