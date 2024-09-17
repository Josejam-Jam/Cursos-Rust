/* Ejer 4  Necesitamos una aplicación que calcule la media aritmética de dos notas enteras
Hay que tener en cuenta que la media puede contener decimales.


*/

use std::io;
use std::str::FromStr;

fn main() {
    let nota1;
    let nota2;
    let media: u32;

    println!("\nIntroduce la primera nota");

    let mut entrada1 = String::new();
    io::stdin().read_line(&mut entrada1).expect("Error en la lectura de datos");

    nota1 = u32::from_str(&entrada1.trim()).unwrap();

    println!("Introduce la segunda nota");

    let mut entrada2 = String::new();
    io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");

    nota2 = u32::from_str(&entrada2.trim()).unwrap();

    media = (nota1 + nota2) / 2;

    //media = get_media(nota1, nota2);

    println!("\nNota media\t{}\n", media);

    //print!("\nNota Media Funcion:\t{}\n", media);
    print!("\nNota Media Funcion:\t{}\n", get_media(nota1, nota2));

}

fn get_media(nota1: u32, nota2: u32) -> u32 {
    return  (nota1 + nota2) / 28;
}

