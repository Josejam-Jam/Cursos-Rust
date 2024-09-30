/*  Ejer 16     Solicitar un número comprendido entre 1 y 99. El programa debe mostrarlo escrito.

                Por ejemplo, para 56 mostrar “cincuenta y seis”.
*/

use std::io;
use std::str::FromStr;

fn main() {
    let mut num: i32;

        loop {
            println!("\nIntroduce un Número entre [ 1 - 99 ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la Lectura de Datos");
            num = i32::from_str(&entrada.trim()).unwrap();

                if num >= 0 && num <= 99 {
                    break;
                } else {
                    println!("\nError al Introducir los datos");
                }
        }

    let unid: i32;
    let dec: i32;
    // let unid: i16;
    // let dec: i16;

    let mut result_tupla: (String, String, String) = ("".to_string(), "".to_string(), "".to_string());

        if num >= 10 && num <= 15 {
            
            result_tupla.0 = match num {
                10 => "diez".to_string(),
                11 => "once".to_string(),
                12 => "doce".to_string(),
                13 => "trece".to_string(),
                14 => "catorce".to_string(),
                15 => "quince".to_string(),
                _ => "".to_string(),
            };
            
        } else {
            unid = get_unidades(num);
            dec = get_decenas(num);

            result_tupla.0 = match dec {
                0 => "".to_string(),
                1 => "dieci".to_string(),
                2 => "veinte".to_string(),
                3 => "treinta".to_string(),
                4 => "cuarenta".to_string(),
                5 => "cincuenta".to_string(),
                6 => "sesenta".to_string(),
                7 => "setenta".to_string(),
                8 => "ochenta".to_string(),
                9 => "noventa".to_string(),
                _ => "".to_string(),
            };

                if dec != 0 && dec != 1 {
                    result_tupla.1 = " y ".to_string();
                } else {
                    result_tupla.1 = "".to_string();
                }

            //result_num_letra = result_num_letra.to_string() + match unid {
            result_tupla.2 = match unid {
                0 => "cero".to_string(),
                1 => "uno".to_string(),
                2 => "dos".to_string(),
                3 => "tres".to_string(),
                4 => "cuatro".to_string(),
                5 => "cinco".to_string(),
                6 => "seis".to_string(),
                7 => "siete".to_string(),
                8 => "ocho".to_string(),
                9 => "nueve".to_string(),
                _ => "".to_string(),
            };
        }

    println!("\nNúmero\t{} en Letras:\t{}{}{}\n", num, result_tupla.0, result_tupla.1, result_tupla.2);

}
/* 
fn get_unidades(num: i32) -> i16 {
    return (num % 10) as i16;
}

fn get_decenas(num: i32) -> i16 {
    return (num / 10) as i16;
}
*/
fn get_unidades(num: i32) -> i32 {
    return num % 10;
}

fn get_decenas(num: i32) -> i32 {
    return num / 10;
}

