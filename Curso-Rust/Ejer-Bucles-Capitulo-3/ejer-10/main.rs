/*      Ejer    10      Se desea implementar una aplicación que pida al usuario que introduzca un número
                        comprendido entre 1 y 10. 
                        
                        Debemos mostrar la tabla de multiplicar de dicho número.

                        El código tendrá que asegurarse de que el número introducido se encuentra entre el 1 y el 10.

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let mut num: i32;
    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok: bool;

        loop {
            println!("\nIntroduce un Número para Mostrar la Tabla de Multiplicar Correspondiente");
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
        
                            num = i32::from_str(&entrada.trim()).unwrap();
        
                            data_ok = exp_regex.is_match(&num.to_string());
        
                                // if exp_regex.is_match(&num.to_string()) {
                                if data_ok {

                                    if num > 0 && num <= 10 /* num != 0 */ /* num > 0 || num < 0 */ {
                                        println!("\nTabla de Multiplicar Número {}\n", num);
                                        show_tabla_multiplicar(num);

                                        break;
                                    }
                                    
                                } else {
                                    println!("\nError al introducir los Datos");
                                }
                        }
                }
        }

}

fn get_multiplicar(num: i32, multiplicando: i32) -> i32 {
    return num * multiplicando;
}

fn show_tabla_multiplicar(num: i32) {
    let mut aux_mult: i32 = 0;

        loop {

            if aux_mult > 10 {  break; }

            if aux_mult <= 10 {
                println!("\t{}\tx\t{}\t=\t{}\n",num, aux_mult, get_multiplicar(num, aux_mult));
            }

            aux_mult += 1;
        }

}
