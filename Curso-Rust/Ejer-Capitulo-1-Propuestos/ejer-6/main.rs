/*  Ejer 6  Diseñar una aplicación que solicite al usuario que introduzca una cantidad de segundos.

            La aplicación debe mostrar cuántas horas, minutos y segundos hay en el número de segundos
            introducidos por el usuario.
*/

use std::io;
use std::str::FromStr;

fn main() {

    let mut cant_seg: i32;
    let horas: i32;
    let min: i32;
    let seg: i32;

    println!("\nIntroduce una cantidad de segundos");
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
    cant_seg = i32::from_str(&entrada.trim()).unwrap();

    horas = get_horas(cant_seg);

    cant_seg = cant_seg - (horas * 3600);
    //cant_seg = cant_seg - horas * 3600;

    min = get_min(cant_seg);

    //cant_seg = cant_seg - (min * 60);

    seg = get_seg(horas, min, cant_seg);

    println!("\nHoras: {}\tMinutos: {}\tSegundos: {}\n", horas, min, seg);

}

fn get_horas(cant_seg: i32) -> i32 {
    return cant_seg / 3600;
}

fn get_min(cant_seg: i32) -> i32 {
    return cant_seg / 60;
}

fn get_seg(cant_horas: i32, cant_min: i32, cant_seg: i32) -> i32 {
    let mut result: i32 = 0;

        if cant_seg % 3600 != 0 {
            result = cant_seg - (cant_horas * 3600);
            //result =- (cant_horas * 3600);
        }

        if cant_min % 60 != 0 {
            result = cant_seg - (cant_min * 60);
            //result =- (cant_min * 60);
        }

    return result; 
}
