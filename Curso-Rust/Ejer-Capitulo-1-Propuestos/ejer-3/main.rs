/*  Ejer 2  Escribir un programa que tome como entrada un número entero y nos indique qué cantidad
            hay que sumarle para que el resultado sea múltiplo de 7

                Ejemplo:
                    * A 2 hay que sumarle 5 para que el resultado (2 + 5 = 7) sea múltiplo de 7
                    * A 13 hay que sumarle 1 para que el resultado (13 + 1 = 14) sea múltiplo
                        de 7
            Si proporcionamos el número 2 o el 13, la salida de la aplicación debe ser 5 o 1 respectivamente      

    Ejer-3  Modificar el ejercicio anterior para que, indicando dos números n y m, nos diga qué cantidad
            hay que sumarle a n para que sea múltiplo de m.
*/

use std::io;
use std::str::FromStr;

fn main() {
    let num1: i32;
    let num2: i32;

    println!("\nIntroduce el primer número");
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
    num1 = i32::from_str(&entrada.trim()).unwrap();

    println!("\nIntrouce el segundo número");
    let mut entrada2 = String::new();

    io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");
    num2 = i32::from_str(&entrada2.trim()).unwrap();

    let num_result = get_sumando_multiplo(num1, num2);

    println!("\nCantidad a sumar:\t{}", num_result);

}

fn get_sumando_multiplo(num1: i32, num2: i32) -> i32 {
    let mut result: i32 = 0;

        if num1 % num2 != 0 { 
            result = num2 - (num1 % num2);
        }

    return result;    
}
