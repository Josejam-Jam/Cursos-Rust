/*  Ejer 3  Solicitar dos números distintos y mostrar cuál es el mayor  */

use std::io;
use std::str::FromStr;

fn main() {
    let num1: i32;
    let num2: i32;

    println!("\nIntroduce un número entero");
    let mut entrada1 = String::new();

    io::stdin().read_line(&mut entrada1).expect("Error en la lectura de datos");
    num1 = i32::from_str(&entrada1.trim()).unwrap();

    println!("\nIntroduce un segundo número entero");
    let mut entrada2 = String::new();

    io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");
    num2 = i32::from_str(&entrada2.trim()).unwrap();


        if num1 > num2 || num2 > num1 {
            let mayor = get_mayor(num1, num2);

            println!("\nLos Números introducidos son\t{}\t{}\tEl mayor es:\t{}\n", num1, num2, mayor);

        } else { 

            println!("\nError al introducir los datos\n");
        }

}

/* 
fn is_distintos(num1: i32, num2: i32) -> bool {
    return num1 != num2;
}
*/

fn get_mayor(num1: i32, num2: i32) -> i32 {
    let result;

        if num1 > num2 {
            result = num1;
        } else {
           result = num2;
        }
    return result;
}
