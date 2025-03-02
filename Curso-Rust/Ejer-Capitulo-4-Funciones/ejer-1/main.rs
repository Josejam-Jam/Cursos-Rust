/*      Ejer  1     Diseñar la función ( fun ) eco ( num: i32 ) a la que se le pasa como parámetros un número n, y
                    muestra por pantalla n veces el mensaje:

                    Eco ...

*/

use std::io;
use std::str::FromStr;
use regex::Regex;


fn main() {
    let mut num: i32;
 
    let exp_regex = Regex::new(r"\d+").unwrap();
    let exp_regex_caracter = Regex::new(r"[a-zA-Z]+").unwrap();
    let exp_regex_white_space = Regex::new(r"\s+").unwrap();

    let mut data_ok : bool;


        loop {
            println!{"\nIntroduce la cantidad del 'Ecos' devolverá la función"};
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");

            let aux_ok = &entrada.is_empty();

                if !*aux_ok  {
                    let aux_data_ok: bool = exp_regex_caracter.is_match(&entrada.trim().to_string()) || exp_regex_white_space.is_match(&entrada.to_string()) && !exp_regex.is_match(&entrada.trim().to_string());

                    if aux_data_ok {
                        println!("\nError al introducir los Datos");
                    }

                    if exp_regex.is_match(&entrada.trim().to_string()) {

                        num = i32::from_str(&entrada.trim()).unwrap();

                        data_ok = exp_regex.is_match(&num.to_string()) && num >= 0;

                            if data_ok {
                                break;
                            }
                    }

                }

        }

        if data_ok {
            /*  FUNCIÓN QUE INCLUYE UN DELAY DE TIEMPO Y MECANISMOS DE SALIDA PARA EVITAR COLAPSOS Y BUCLES INFINITOS    */
            //get_eco_repetitions(num);

            /*  FUNCIÓN QUE DEVUELVE EL MENSAJE ECO N VECES INTRODUCIDAS POR PARÁMETRO    */

            eco(num);
        }

}

/*  FUNCIÓN QUE DEVUELVE ALERTAS SOBRE INTENCIONES DE COLAPSOS EN LA APP Y PREVEE BUCLES INFINITOS    */
fn get_alert_intentions(ind_iterador: i32) {

    if ind_iterador == 2000 {
        print!("\tAtent@ NO molan los 'ECOS' innecesarios\n\n");
    }

    if ind_iterador == 3000 {
        print!("\tNO SE PERMITEN LOS BÚCLES INFINITOS, YA HAS COMPROBADO QUE FUNCIONA, SE TERMINÓ\n\n");
    }

}
/*  FUNCIÓN QUE DEVUELVE UN DELAY DE TIEMPO PARA LA ITERACIÓN    */
/*fn get_delay_print() -> i32 {
    return 1000;
}
*/

/*  FUNCIÓN QUE INCLUYE UN DELAY DE TIEMPO Y MECANISMOS DE SALIDA PARA EVITAR COLAPSOS Y BUCLES INFINITOS   */
/*fn get_eco_repetitions(num_iteraciones: i32) {

    let aux_iter_delay = get_delay_print();
    let aux_iterador = num_iteraciones * 1000;

        for i in 0..aux_iterador {
            
            if  i % aux_iter_delay == 0 {
                print!("\nEco...");
                get_alert_intentions(i);

                    if i == 3000 {
                        break;
                    }

            } else {
                if i % 200 == 0 {
                    println!("\nEco...\n");
                }
            } 
        }
}
*/

/*  FUNCIÓN QUE IMPRIME UN ECO UN NUMERO DE VECES INTRODUCIDAS POR PARÁMETRO    */
fn eco(num_iteraciones: i32) {

        for i in 0..num_iteraciones {
            
            if i == 2000 || i == 3000 {
                print!("\nEco...");
                get_alert_intentions(i);

                    if i == 3000  {
                        break;
                    }

            } else {
                println!("\nEco...\n");
            } 
        }
}
