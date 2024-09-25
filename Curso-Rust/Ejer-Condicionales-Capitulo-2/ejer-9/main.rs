/*  Ejer 9  Escribir un aplicación que indique cuántas cifras tiene un número entero introducido por
            teclado, que estará comprendido entre 0 y 99999.   
*/

use std::io;
use std::str::FromStr;

fn main() {
    let mut num: i32;
    let cant_cifras: i32;

        loop {
            println!("\nIntroduce un Número Entero entre [ 0 - 99999 ]");
            let mut entrada1 = String::new();

            io::stdin().read_line(&mut entrada1).expect("Error en la Lectura de Datos");
            num = i32::from_str(&entrada1.trim()).unwrap();

                if num >= 0 && num <= 99999 {
                    cant_cifras = get_cant_cifras(num);
                    break; 
                } else {
                    println!("\nError el número introducido no está dentro del rango");
                }
        }

    println!("\nEl Número\t{}  tiene\t{}\tcifras\n", num, cant_cifras);
}

fn get_cant_cifras(num: i32) -> i32 {
    let mut result: i32 = 0;

        if num < 10 {
            result = 1;
        } else if num > 9 && num <= 99 {
            result = 2;
        } else if num > 99 && num <= 999 {
            result = 3;
        } else if num > 999 && num <= 9999 {
            result = 4;
        } else if num > 9999 && num <= 99999 {
            result = 5;
        }

    return result;      
}
