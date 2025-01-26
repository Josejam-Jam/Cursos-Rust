/*  Ejer 9  Diseñar una aplicación que calcule la longitud y el área de una circunferencia, para ello, el
            usuario debe introducir el radio que puede contener decimales:

                formulas: longitud = 2PI * r²
                área = PI * r²
*/

use std::io;
use std::str::FromStr;

fn main() {

    const PI: f32 = std::f32::consts::PI; 
    //const PI: f64 = std::f64::consts::PI; 
    //let pi: f32 = std::f32::consts::PI; 

    let radio: f32;
    let longitud: f32;
    let area: f32;

    println!("\nIntroduce el rádio");
    
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error en la lectura");

    radio = f32::from_str(&entrada.trim()).unwrap();

    //longitud = get_long_circunferencia(radio, pi);
    longitud = get_long_circunferencia(radio, PI);

    //area = get_area_circunferencia(radio, pi);
    area = get_area_circunferencia(radio, PI);

    println!("\n Longitud:\t{}\n", longitud);

    println!("\n Área:\t{}\n", area);

}

fn get_long_circunferencia(radio: f32, pi: f32) ->f32 {
    return (2.0 * pi) * radio.powi(2);
}

fn get_area_circunferencia(radio: f32, pi: f32) ->f32 { 
    return pi * radio.powi(2);
}