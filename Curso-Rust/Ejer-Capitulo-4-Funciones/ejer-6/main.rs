/*      Ejer    6   Crear una función que, mediante un booleano, indique si el carácter que se pasa como
                    parámetro de entrada corresponde con una vocal.

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {

        let mut caracter : String;
    
        let exp_regex = Regex::new(r"\d+").unwrap();
        let exp_regex_caracter = Regex::new(r"[a-zA-Z]{1}").unwrap();
        let exp_regex_white_space = Regex::new(r"\s+").unwrap();

        let exp_regex_vocal = Regex::new(r"[aeiouáéíóúAEIOUÁÉÍÓÚäëïöüÄËÏÖÜ]{1}").unwrap();
    
        let mut data_ok: bool;
     
            loop {
                println!("\nIntroduce un Caracter || Letra");
                let mut entrada = String::new();
    
                io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
    
                let aux_ok = &entrada.is_empty();
    
                    if !*aux_ok {
                        let aux_data_ok: bool = (!exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) )
                            && exp_regex.is_match(&entrada.trim().to_string()) || !exp_regex_vocal.is_match(&entrada.trim().to_string()) || entrada.to_string().len() > 1;
    
                        if aux_data_ok {
                            println!("\nError al introducir los Datos");
                        }
    
                        if exp_regex_caracter.is_match(&entrada.trim().to_string()) {
                            caracter = String::from_str(&entrada.trim()).unwrap();
    
                            data_ok = (exp_regex_vocal.is_match(&caracter.trim().to_string()) || exp_regex_caracter.is_match(&caracter.trim().to_string())) && caracter.to_string().len() == 1;
    
                                if data_ok {
                                    break;
                                }
                        }

                    }
            }

            if data_ok {
    
                println!("\n\tEl Carácter introducido es Vocal:\t{}\n", is_vocal(caracter));
                
            }
}

/*  FUNCIÓN QUE RECIBA UN SÓLO CARÁCTER Y DEVUELVA [ TRUE || FALSE ] SI ES VOCAL     */
fn is_vocal(caracter: String) -> bool {
    let exp_regex = Regex::new(r"[aeiouáéíóúAEIOUÁÉÍÓÚäëïöüÄËÏÖÜ]{1}").unwrap();

    return exp_regex.is_match(&caracter.trim().to_string());
}
    