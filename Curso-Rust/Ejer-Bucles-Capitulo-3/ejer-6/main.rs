/*      Ejer    6       Escribir todos lo múltiplos de 7 menores que 100.       */

/* 
use std::io;
use std::str::FromStr;
use regex::Regex;
*/

fn main() {
    const NUM_STAND: i32 = 7;
    const NUM_MAX: i32 = 100;


    println!("\nMostramos los Múltiplos de {}\tMenores que {}\n", NUM_STAND, NUM_MAX);

    show_multiplos_number(NUM_STAND, NUM_MAX);

    println!("");

}

fn show_multiplos_number(num_stand: i32, num_max: i32) {
    let mut aux_num = num_stand;

        loop {

            if aux_num > num_max {
                break;
            } else {
                println!("\t{}", aux_num);
            }
            aux_num += num_stand;
        }
}
