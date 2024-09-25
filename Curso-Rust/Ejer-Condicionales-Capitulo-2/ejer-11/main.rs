/*  Ejer 11     Pedir una nota de 0 a 10 y mostrarla de la forma: Insuficiente (de 0 a 4), Suficiente (5), Bien
                (6), Notable (7 y 8) y Sobresaliente (9 y 10).  
*/

use std::io;
use std::str::FromStr;

fn main() {
    let nota: f32;

        loop {

            println!("\nIntroduce nota");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
            nota = f32::from_str(&entrada.trim()).unwrap();

                if nota >= 0.0 && nota <= 10.0 {
                    break;
                } else {
                    println!("\nError al introducir los datos");
                }
        }

    

    get_nota(nota);
}

fn get_nota(nota: f32) {
    let mut result_nota = String::new();
    //let mut result_nota : String = String::from("");

        if nota >= 0.0 && nota <= 4.9 {
            // result_nota = String::from("INSUFICIENTE");
            result_nota = "INSUFICIENTE".to_string();
        }

        if nota >= 5.0 && nota <= 5.9 {
            // result_nota = String::from("SUFICIENTE");
            result_nota = "SUFICIENTE".to_string();

        }

        if nota >= 6.0 && nota <= 6.9 {
            // result_nota = String::from("BIEN");
            result_nota = "BIEN".to_string();
        }

        if nota >= 7.0 && nota <= 8.9 {
            // result_nota = String::from("NOTABLE");
            result_nota = "NOTABLE".to_string();
        }

        if nota >= 9.0 && nota <= 10.0 {
            // result_nota = String::from("SOBRESALIENTE");
            result_nota = "SOBRESALIENTE".to_string();
        }

    //print!("{}",result_nota.to_string());
    println!("\nCalificación\t{}\t{}\n", nota, result_nota);
}
