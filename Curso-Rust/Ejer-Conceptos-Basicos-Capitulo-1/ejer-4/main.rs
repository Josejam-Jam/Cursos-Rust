/* Ejer 4  Necesitamos una aplicación que calcule la media aritmética de dos notas enteras
            Hay que tener en cuenta que la media puede contener decimales.
*/

use std::io;
use std::str::FromStr;

fn main() {
    let nota1;
    let nota2;
    //let media: u32;
    let media: f32;

    println!("\nIntroduce la primera nota");

    let mut entrada1 = String::new();
    io::stdin().read_line(&mut entrada1).expect("Error en la lectura de datos");

    //nota1 = u32::from_str(&entrada1.trim()).unwrap();
    nota1 = f32::from_str(&entrada1.trim()).unwrap();

    println!("Introduce la segunda nota");

    let mut entrada2 = String::new();
    io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");

    //nota2 = u32::from_str(&entrada2.trim()).unwrap();
    nota2 = f32::from_str(&entrada2.trim()).unwrap();

    //media = (nota1 + nota2) / 2;
    media = (nota1 + nota2) / 2.0;

    //media = get_media(nota1, nota2);

    println!("\nNota media\t{}\n", media);

    //print!("\nNota Media Funcion:\t{}\n", media);
    //print!("\nNota Media Funcion:\t{}\n", get_media(nota1, nota2));

    print!("\nNota Media Float Funcion:\t{}\n", get_media_float(nota1, nota2));

}
/** Función para devolver la media sin decimales en un entero */
/* 
fn get_media(nota1: u32, nota2: u32) -> u32 {
    return  (nota1 + nota2) / 2;
}
*/
/** Función calcula la media exacta con decimales */
fn get_media_float(nota1: f32, nota2: f32) -> f32 {
    return  (nota1 + nota2) / 2.0;
}

