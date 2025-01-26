/*  Ejer 6  Pedir dos números y mostrarlos ordenados de forma decreciente.  */

use std::io;
use std::str::FromStr;

fn main() {
    let num1: i32;
    let num2: i32;

    println!("\nIntroduce un número");
    let mut entrada1 = String::new();

    io::stdin().read_line(&mut entrada1).expect("Error en la lectura de datos");
    num1 = i32::from_str(&entrada1.trim()).unwrap();

    println!("\nIntroduce un Segundo Número");
    let mut entrada2 = String::new();

    io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");
    num2 = i32::from_str(&entrada2.trim()).unwrap();

        if num1 > num2 { 
            println!("\nMostramos los Números Ordenados de Forma Decreciente\t{}\t{}\n", num1, num2);

        } else if num1 < num2 { 
            println!("\nMostramos los Números Ordenados de Forma Decreciente\t{}\t{}\n", num2, num1);

        } else {
            if num1 == num2{
                println!("\nLos Números son iguales Mostramos el Orden de entrada de Datos\t{}\t{}\n", num1, num2);
            }
        }
}
