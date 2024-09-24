/*  Ejer 1  Diseñar una aplicación que solicite al usuario un numero e indique si es par o impar    */

use std::io;
use std::str::FromStr;

fn main() {
    let num: i32;
    let par: bool;

    println!("\nIntroduce un Número");
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
    num = i32::from_str(&entrada.trim()).unwrap();

    par = is_par(num);

    men_respuesta(num, par);
}

fn is_par(num: i32) -> bool {
    return num % 2 == 0;
}

fn men_respuesta(num: i32, is_par: bool) {
    let result;

        if is_par {
            result = String::from("Es Par");
        } else {
            result = String::from("Es Impar");
        }

    println!("\nEl Número\t{}\t{}\n", num, result);
    //print!("El Número\t{}\t{}", num, result);
}
