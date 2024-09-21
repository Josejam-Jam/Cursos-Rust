/*  Ejer-4  Crear un programa que pida la base y la altura de un triángulo y muestre su área
            
            A = (b * h) / 2 
*/

use std::io;
use std::str::FromStr;

fn main() {

    let altura: f32;
    let base: f32;
    let area: f32;

    println!("\nIntroduce la Base del Triángulo");
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
    base = f32::from_str(&entrada.trim()).unwrap();

    println!("\nIntroduce la Altura del Triángulo");
    let mut entrada2 = String::new();

    io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");
    altura = f32::from_str(&entrada2.trim()).unwrap();

    area = get_area_triangulo(base, altura);

    println!("\nEl Área del Triangulo es:\t{}", area);
}

fn get_area_triangulo(base: f32, altura: f32) -> f32{

    return (base * altura) / 2.0;

}
