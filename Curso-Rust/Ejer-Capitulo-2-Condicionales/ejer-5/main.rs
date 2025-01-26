/*  Ejer 5  Implementar un programa que pida por teclado un número decimal e indique
            si es un número casi-cero, que son aquellos, positivos o negativos, que se acercan a 0
            por menos de 1 unidad, aunque curiosamente el 0 no se considera un número casi-cero

                Ejemplos de números casi-cero son el 0.3, el -0.99 o el 0.123

            Y números que no se consideran casi-ceros son: el 12.3, el 0 o el -1.
*/

use std::io;
use std::str::FromStr;

fn main() {
    let num1: f32;

    println!("\nIntroduce un Número Decimal");
    let mut entrada1 = String::new();

    io::stdin().read_line(&mut entrada1).expect("Error en la lectura de datos");
    num1 = f32::from_str(&entrada1.trim()).unwrap();

    let es_casi_cero = is_casi_cero(num1);

        if es_casi_cero {
            println!("\nEl Número\t{}\t es Casi-Cero\n", num1);
        } else {
            println!("\nEl Número\t{}\t es No Casi-Cero\n", num1);

        }
}

fn is_casi_cero(num: f32) -> bool {
    let mut result: bool = false;

        if num >= 0.1 && num <= 0.99  {
            result = true;
        }

        if num >= -0.99 && num <= -0.1 {
            result = true;
        }

    return result;
}
