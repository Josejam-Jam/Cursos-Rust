/*      Ejer 1      Diseñar un programa que muestre, para cada número introducido por teclado, si es par,
                    si es positivo y su cuadrado.

                    El proceso se repetirá hasta que el número introducido por teclado sea 0.

*/

use std::io;
use std::str::FromStr;

fn main() {

    let mut num: i32;

        loop {

            println!("\nIntroduce un número por teclado\n");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
            num = i32::from_str(&entrada.trim()).unwrap();

            let es_par:bool = is_par(num);
            let es_positivo: bool = is_positive(num);

                if num != 0 {
                    if es_par && es_positivo {

                        println!("\nEl número {} es par, positivo y su cuadrado es {}\n", num, get_cuadrado(num));
                    }
                    
                    if es_par && !es_positivo {
    
                        println!("\nEl número {} es par, negativo y su cuadrado es {}\n", num, get_cuadrado(num));
                    }
                    
                    if !es_par && es_positivo{
    
                        println!("\nEl número {} es inpar, positivo y su cuadrado es {}\n", num, get_cuadrado(num));
                    }
                    
                    if !es_par && !es_positivo {
    
                        println!("\nEl número {} es impar, negativo y su cuadrado es {}\n", num, get_cuadrado(num));
                    } 
                }

                if num == 0 {
                    break;
                } 
        }

}

fn is_par(num: i32) -> bool {
    return num % 2 == 0;
}

fn is_positive(num: i32) -> bool { 
    return num > 0;
}

fn get_cuadrado(num: i32) -> i32 {
    return num.pow(2);
}