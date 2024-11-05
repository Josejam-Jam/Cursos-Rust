/*      Ejer 4      Escribir una aplicación para aprender a contar, que pedirá un número n y mostrará todos lo
                    números del 1 a n.
*/

use std::io;
use std::str::FromStr;
use regex::Regex;


fn main() {
    let mut num: i32;
    let exp_regex = Regex::new(r"\d+").unwrap();
    let mut datos_ok: bool;

        loop {
            println!("\nIntroduce un número");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            num = i32::from_str(&entrada.trim()).unwrap();
            datos_ok = exp_regex.is_match(&num.to_string());

                // if exp_regex.is_match(&num.to_string()) {
                if datos_ok {
                    datos_ok = true;
                    break;
                } else {
                    println!("\nError al introducir los Datos Introduzca un número Positivo");
                }
        }

        if datos_ok {
            println!("\nLos Números entre [ 1 - {} ]\n", num);
            show_range_numbers(num);
        }
    
}

fn show_range_numbers(num: i32) {
    let mut init = 0;

        loop {
            init += 1;
            println!("{}", init);

                if init == num {
                    // println!("\n");
                    println!("");
                    break;
                }
        }
}
