/*  Ejer 3  Escribir una aplicación que pida el año actual y el de nacimiento del usuario.
            Debe calcular su edad, suponiendo que en el año en curso el usuario ya ha cumplido años.

*/

use std::io;
use std::str::FromStr;
//use std::fmt::Display;

fn main() {

    let year_nacimiento;
    let year_actual;
    let edad;
    let salida_funcion;

    println!("\nIntroduce el año en que naciste");
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect("Error en la entrada de datos");
    //io::stdin().read_line(&mut entrada).unwrap();

    year_nacimiento = u32::from_str(&entrada.trim()).unwrap();

    println!("\nIntroduce el año Actual");

    let mut entrada2 = String::new();

    io::stdin().read_line(&mut entrada2).expect("Error en la entrada de datos");
    //io::stdin().read_line(&mut entrada2).unwrap();

    year_actual = u32::from_str(&entrada2.trim()).unwrap();

    edad = year_actual - year_nacimiento;

    println!("\nTú edad actual es {} años\n", edad);

    salida_funcion = get_edad_actual(year_actual, year_nacimiento);

    println!("\nCalculo de la Edad Función: {} años ", salida_funcion);

    println!("\nCalculo edad Función Implicita en la salida : {} años \n", get_edad_actual(year_actual, year_nacimiento));


    get_edad_actual_void(year_actual, year_nacimiento);

}

fn get_edad_actual(year_actual: u32, year_nacimiento: u32) -> u32 {
    let result = { year_actual - year_nacimiento };
    
    return result;
    
    //return yearactual - fnacimiento;

}

fn get_edad_actual_void(year_actual: u32, year_nacimiento: u32) {
    let result = { year_actual - year_nacimiento };

    println!("\nCalculo de la Edad Función Void: {} años \n", result);
    // print!("\nCalculo de la Edad Función Void:\t");
    // println!("{} años\n",  result);
}


