/*  Ejer 10     Pedir un número entre 0 y 99999, y decir si es capicúa.   */

use std::io;
use std::str::FromStr;

fn main() {
    let mut num: i32;

        loop {
            println!("\nIntroduce un Número Entero entre [ 0 - 99999 ]");

            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
            num = i32::from_str(&entrada.trim()).unwrap();

                if num > 0 && num <= 99999 {
                    break;   
                } else {
                    println!("\nError el Número Introducido está fuera del Rango");
                }
        }

    let es_capicua = is_capicua(num);

        if es_capicua {
            println!("\nEl Introducido\t{}\tes\tCapicúa\n", num);
        } else {
            println!("\nEl Introducido\t{}\tNo es\tCapicúa\n", num);

        }

}

// Función que segmenta el número y devuelve una tupla
fn get_segmentacion_tupla(num: i32) -> (i32, i32, i32, i32, i32) { 
    let mut num_aux: i32 = num;

    let dm: i32 = num % 10;
    num_aux = num_aux / 10;

    let udm: i32 = num % 10;
    num_aux = num_aux / 10;

    let cent: i32 = num % 10;
    num_aux = num_aux / 10;

    let dec: i32 = num % 10;
    num_aux = num_aux / 10;

    let unid = num_aux;

    let result_tupla = (dm, udm, cent, dec, unid);

    return result_tupla;
}

// Función que segmenta el número y devuelve un array
/* fn get_segmentacion_array(num: i32) -> [i32; 5] { 
    let mut num_aux: i32 = num;

    let dm: i32 = num % 10;
    num_aux = num_aux / 10;

    let udm: i32 = num % 10;
    num_aux = num_aux / 10;

    let cent: i32 = num % 10;
    num_aux = num_aux / 10;

    let dec: i32 = num % 10;
    num_aux = num_aux / 10;

    let unid = num_aux;

    let result_array: [i32; 5] = [dm, udm, cent, dec, unid];

    return result_array;
}
*/

// Función que segmenta el número y devuelve un vector

/*fn get_segmentacion_vector(num: i32) -> Vec<i32> {
    let mut num_aux: i32 = num;

    let dm: i32 = num % 10;
    num_aux = num_aux / 10;

    let udm: i32 = num % 10;
    num_aux = num_aux / 10;

    let cent: i32 = num % 10;
    num_aux = num_aux / 10;

    let dec: i32 = num % 10;
    num_aux = num_aux / 10;

    let unid = num_aux;

    let mut result_vector = Vec::new();

    result_vector.push(dm);
    result_vector.push(udm);
    result_vector.push(cent);
    result_vector.push(dec);
    result_vector.push(unid);

    return result_vector;
}
*/

fn is_capicua(num: i32) -> bool {
    let mut result = false;

    // Función que segmenta el número y devuelve un tupla
    let num_segmentado_tupla = get_segmentacion_tupla(num);
    

        if num_segmentado_tupla.0 != 0 
            && num_segmentado_tupla.0 == num_segmentado_tupla.4 
            && num_segmentado_tupla.1 == num_segmentado_tupla.3 {

                result = true;
        }


    // Función que segmenta el número y devuelve un array
    /*let num_segmentado_array = get_segmentacion_array(num);

        if num_segmentado_array[0] != 0 
            && num_segmentado_array[0] == num_segmentado_array[4] 
            && num_segmentado_array[1] == num_segmentado_array[3] {

                result = true;
        }

    */

    // Función que segmenta el número y devuelve un vector
    /*let num_segmentado_vector: Vec<i32> = get_segmentacion_vector(num);

        if num_segmentado_vector[0] != 0 
            && num_segmentado_vector[0] == num_segmentado_vector[4] 
            && num_segmentado_vector[1] == num_segmentado_vector[3] {

                result = true;
        }
    */

    return result;    
}
