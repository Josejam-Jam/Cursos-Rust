/*      Ejer 2      En una granja se compra diariamente una cantidad (comida_diaria) de comida para los
                    animales. El número de animales a alimentar (todos de la misma especie) es num_animales,
                    y sabemos que cada animal come una media de kilos_por_animal.

                    Diseñar un programa que solicite al usuario los valores anteriores y determine si disponemos
                    de alimento suficiente para cada animal. En caso negativo, ha de calcular cuál es la ración que
                    corresponde a cada uno de los animales.


                    Nota: Evitar que la aplicación realice divisiones por cero.
*/

use std::io;
use std::str::FromStr;
use regex::Regex;

fn main() {

    let mut num_animales: i32;
    let mut comida_diaria: f32;
    let mut kilos_por_animal: f32;
    let cant_comida_necesaria: f32;
    
    let exp_regex = Regex::new(r"\d+\.\d+").unwrap();
    let exp_regex2 = Regex::new(r"\d+").unwrap();

        loop {

            println!("\nIntroduce la Cantidad de Comida la que disponemos");
            let mut entrada = String::new();

            io::stdin().read_line(&mut entrada).expect("Error en la lectura de datos");
            comida_diaria = f32::from_str(&entrada.trim()).unwrap();
            
                if exp_regex.is_match(&comida_diaria.to_string()) && comida_diaria > 0.0 {
                    break;
                } else {
                    println!("\nError al introducir los datos\n");
                }
        }

        loop {

            println!("\nIntroduce la Cantidad de Animales que tenemos");
            let mut entrada2 = String::new();

            io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");
            num_animales = i32::from_str(&entrada2.trim()).unwrap();

                if exp_regex2.is_match(&num_animales.to_string()) && num_animales > 0 {
                    break;
                } else {
                    println!("\nError al introducir los datos\n");
                }
        }

        loop {

            println!("\nIntroduce la Ración en Kilos por Animal");
            let mut entrada3 = String::new();

            io::stdin().read_line(&mut entrada3).expect("Error en la lectura de datos");
            kilos_por_animal = f32::from_str(&entrada3.trim()).unwrap();

                if exp_regex.is_match(&kilos_por_animal.to_string()) && kilos_por_animal > 0.0 {
                    break;
                } else {
                    println!("\nError al introducir los datos\n");
                }
        }

    cant_comida_necesaria = get_cant_necesaria_diaria_total(num_animales, kilos_por_animal);    

    let hay_comida_suficiente: bool =  get_hay_suficiente_comida(comida_diaria, cant_comida_necesaria);

        if !hay_comida_suficiente {
            let aux_cant_falta = get_cant_necesaria_real(comida_diaria, cant_comida_necesaria);

            println!("\nNo hay Suficiente comida para cumplir con la ración diaria por animal\t{}\tkg/Animal\nDEBERÍAN COMPRARSE\t{}\tkg\n",kilos_por_animal, aux_cant_falta);

            let aux_racion_diaria = get_racion_diaria_comida_animal(comida_diaria, num_animales);

            println!("\nLa ración correspondiente en este caso es:\t{}\tkg\\animal\n", aux_racion_diaria);

        } else {
            println!("\nComida necesaria\t{}\tkg\n", cant_comida_necesaria);
            println!("\nHay comida suficiente para los Animales\n");
        }

}

fn get_cant_necesaria_diaria_total(num_animales: i32, kilos_por_animal: f32) -> f32 {
    return num_animales as f32 * kilos_por_animal;
}

fn get_hay_suficiente_comida(comida_diaria: f32, cant_comida_necesaria: f32) -> bool {
    let mut result = false;

        if comida_diaria > 0.0 && cant_comida_necesaria > 0.0 {

            if comida_diaria > cant_comida_necesaria {  result = true; }

        }
        
    return result;    
}

fn get_cant_necesaria_real(comida_diaria: f32, cant_comida_necesaria: f32) -> f32 {
    return (comida_diaria - cant_comida_necesaria) * -1.0;
}
/* 
fn get_racion_diaria_comida(comida_diaria: f32, cant_comida_necesaria: f32) -> f32 {
    return comida_diaria / cant_comida_necesaria;
}
*/
fn get_racion_diaria_comida_animal(comida_diaria: f32, num_animales: i32) -> f32 {
    return (comida_diaria / num_animales as f32) as f32;
}


