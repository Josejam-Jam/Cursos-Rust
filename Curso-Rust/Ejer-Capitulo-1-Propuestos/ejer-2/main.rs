/*  Ejer 2  Escribir un programa que tome como entrada un número entero y nos indique qué cantidad
            hay que sumarle para que el resultado sea múltiplo de 7

                Ejemplo:
                    * A 2 hay que sumarle 5 para que el resultado (2 + 5 = 7) sea múltiplo de 7
                    * A 13 hay que sumarle 1 para que el resultado (13 + 1 = 14) sea múltiplo
                        de 7
            Si proporcionamos el número 2 o el 13, la salida de la aplicación debe ser 5 o 1 respectivamente
*/

use std::io;
use std::str::FromStr;

fn main() {
    const MULTIPLO: i32 = 7;
    let num: i32;

    println!("\nIntroduce un número");
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
    num = i32::from_str(&entrada.trim()).unwrap();

    let num_result = get_sumando_multiplo(num, MULTIPLO);

    println!("\nCantidad a sumar:\t{}", num_result);


}

fn get_sumando_multiplo(num: i32, multiplo: i32) -> i32 {
    let mut result: i32 = 0;

        if num % multiplo != 0 { 
            result = multiplo - (num % multiplo);
        }

    return result;    
}
