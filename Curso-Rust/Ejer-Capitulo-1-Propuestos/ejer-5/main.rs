/*  Ejer-5  Dado un polinomio de segundo grado
        
            y = ax² + bx + c
            
            Crear un programa que pida los coeficientes a , b y c, así como el valor de x, y calcule el
            valor de y.
*/

use std::io;
use std::str::FromStr;

fn main() {
    let coefi_a: f32;
    let coefi_b: f32;
    let coefi_c: f32;
    let coefi_x: f32;
    let valor_y: f32;

    println!("\nIntroduce el valor Coeficiente a");
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
    coefi_a = f32::from_str(&entrada.trim()).unwrap();

    println!("\nIntroduce el valor Coeficiente b");
    let mut entrada2 = String::new();

    io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");
    coefi_b = f32::from_str(&entrada2.trim()).unwrap();

    println!("\nIntroduce el valor Coeficiente c");
    let mut entrada3 = String::new();

    io::stdin().read_line(&mut entrada3).expect("Error en la lectura de datos");
    coefi_c = f32::from_str(&entrada3.trim()).unwrap();

    println!("\nIntroduce el valor Coeficiente x");
    let mut entrada4 = String::new();

    io::stdin().read_line(&mut entrada4).expect("Error en la lectura de datos");
    coefi_x = f32::from_str(&entrada4.trim()).unwrap();

    valor_y = get_value_polinomio_y(coefi_a, coefi_x, coefi_b, coefi_c);

    println!("\nY:\t{}", valor_y);

}

fn get_value_polinomio_y(a: f32, x: f32, b: f32, c: f32) -> f32 {
    return a * (x.powi(2)) + (b * x ) + c;
}
