/*      Ejer 1      Implementar un programa que pida al usuario un número de tres cifras y lo muestre guarismo
                    a guarismo.

                    Por ejemplo, para el número 123, debe mostrar primero el 1, a continuación el 2 y por
                    último el 3.

*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {

    let mut num: i32;
 
    let exp_regex = Regex::new(r"\d{3}").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok : bool;
 
    loop {
        println!("\nIntroduce un Número de Tres Cifras");
        let mut entrada = String::new();

        io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

        let aux_ok = &entrada.is_empty();


            if !*aux_ok {
                let aux_data_ok:bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                    if aux_data_ok {
                        println!("\nError al introducir los Datos");
                    }

                    if exp_regex.is_match(&entrada.trim().to_string()) {

                        num = i32::from_str(&entrada.trim()).unwrap();

                        data_ok = exp_regex.is_match(&num.to_string()) && num > 0;

                            if data_ok {
                                let num_lenght: u32 = num.to_string().len() as u32;

                                println!("\nMostramos el Número\t{}\tguarísmo a guarísmo\n", num);

                                show_guasismos_num(num, num_lenght);

                                /*  *** CASUÍSTCA FORZADA A NÚMERO DE 3 CIFRAS  */
                                // show_guasismos_num_3_cifras(num);
                                
                                break;
                            }
                    }
            }

    }

}

fn show_guasismos_num(num: i32, num_lenght: u32) {
    let aux_length: u32 = num_lenght - 1;
    let base: i32 = 10;
    let mut aux: i32 = base.pow(aux_length);
    let mut aux_num: i32 = num;

        for _i in 0..num {

            if aux == 0 { break; } 

            println!("\n{}\n", aux_num / aux);

            aux_num = aux_num - ((aux_num / aux) * aux);

            aux /= 10;

        }
}

/*  *** CASUÍSTCA FORZADA A NÚMERO DE 3 CIFRAS  */
/*  fn show_guasismos_num_3_cifras(num: i32) {
    //let aux_length: u32 = 3;
    let base: i32 = 10;
    // let mut aux: i32 = base.pow(aux_length);
    let mut aux: i32 = base.pow(3);
    let mut aux_num: i32 = num;

        for i in 0..num {
            
            if aux == 0 { break; } 

            println!("\n{}\n", aux_num / aux);

            aux_num = aux_num - ((aux_num / aux) * aux);

            aux /= 10; 

        }
}
*/
