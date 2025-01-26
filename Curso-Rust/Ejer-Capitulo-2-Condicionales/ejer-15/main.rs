/*  Ejer 15     Idear un programa que solicite al usuario un número comprendido entre 1 y 7,
                correspondiente a un día de la semana. Se debe mostrar el nombre del día de la semana al que
                corresponde. 
                
                Por ejemplo, el número 1 corresponde a “lunes” y el 6 a “sábado”. 

*/

use std::io;
use std::str::FromStr;

fn main() {

    let mut dia: i32;

        loop {

            println!("\nIntroduce el Día de la Semana entre [ 1 - 7 ]");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la Lectura de Datos");
            dia = i32::from_str(&entrada.trim()).unwrap();

                if dia >= 1 && dia <= 7{
                    break;
                } else {
                    println!("\nError al Introducir los datos\n");
                }
        }

    let nomb_dia_semana = get_nombre_dia_semana(dia);

    println!("\nDia de la Semana\t{}\n", nomb_dia_semana);
}

fn get_nombre_dia_semana(dia_semana: i32) -> String {
    // let mut result_dia = String::new();
    let result_dia;

    result_dia  =  match dia_semana {
            1 => "Lunes".to_string(),
            2 => "Martes".to_string(),
            3 => "Miércoles".to_string(),
            4 => "Jueves".to_string(),
            5 => "Viernes".to_string(),
            6 => "Sábado".to_string(),
            7 => "Domingo".to_string(),
            _ => "Lunes".to_string(),
        };

    return result_dia;
}

