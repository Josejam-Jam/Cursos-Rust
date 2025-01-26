/*      Ejer    14      Pedir por consola un número n y dibujar un triángulo rectángulo de na elementos de lado,
                        utilizando para ello asteriscos ( * )
                        
                        Por ejemplo, para n = 4

                            * * * *
                            * * *
                            * *
                            *       

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    const ELEM_STAND_LADO: char = '*';
    let mut num_elem: i32;

    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok: bool;

        loop {
            println!("\nIntroduce el Tamaño/Elementos N de lado del Triángulo");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok {

                    let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());
                    // let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada.trim().to_string());

                    // if exp_regex_caracter.is_match(&entrada.trim().to_string()) {
                        if aux_data_ok {
                            println!("\nError al introducir los Datos Introduzca un número");
                        } 
                        
                        if exp_regex.is_match(&entrada.trim().to_string())  {
        
                            num_elem = i32::from_str(&entrada.trim()).unwrap();
        
                            data_ok = exp_regex.is_match(&num_elem.to_string());
        
                                // if exp_regex.is_match(&num_elem.to_string()) {

                                if data_ok {

                                    // show_triangulo_rectangulo(num_elem, ELEM_STAND_LADO.to_string() + " ");
                                    let aux_elem_stand = ELEM_STAND_LADO.to_string() + " ";
                                    show_triangulo_rectangulo(num_elem, aux_elem_stand);
                                    // show_triangulo_rectangulo(num_elem, "* ".to_string());

                                    break;

                                } else {
                                    println!("\nError al introducir los Datos");
                                }
                        }
                }
        }
}

fn get_triangulo_rectangulo_string(num_elem: i32, elem_stand_lado: String) -> String {
    let mut result: String = String::new();

        for i in 0..num_elem {

            let mut aux_elem_lado = num_elem - i;
            
                loop {

                    result += &elem_stand_lado;

                    aux_elem_lado -= 1;

                        if aux_elem_lado == 0 {
                            break;
                        }
                }
            result += "\n";
        }

    return result;

}

fn show_triangulo_rectangulo(num_elem: i32, elem_stand_lado: String) {

    let result = get_triangulo_rectangulo_string(num_elem, elem_stand_lado);

    println!("\n{}", result);

}
