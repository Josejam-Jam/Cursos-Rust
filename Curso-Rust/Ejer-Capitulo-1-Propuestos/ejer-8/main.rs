/*  Ejer 8  Un biólogo está realizando un estudio de distintas especies de invertebrados y necesita una
            aplicación que le ayude a contabilizar el número de patas que tienen en total todos los animales
            capturados durante una jornada de trabajo.

            Para ello, nos ha solicitado que escribamos una aplicación a la que hay que proporcionar:
                * el número de hormigas capturadas (6 patas)
                * el número de arañas capturadas (8 patas)
                * el número de cochinillas capturadas (14 patas)

            La aplicación debe mostrar el número total de patas que poseen todos los animales. 
*/

use std::io;
use std::str::FromStr;

fn main() {
    const NUM_PATAS_HORMIGA: i32 = 6;
    const NUM_PATAS_SPIDER: i32 = 8;
    const NUM_PATAS_COCHINILLA: i32 = 14;

    let cant_horm: i32;
    let cant_spiders: i32;
    let cant_cochinillas: i32;
    let cant_total_patas: i32;

    println!("\nIntroduce la cantidad de Hormigas capturadas");
    let mut entrada1 = String::new();

    io::stdin().read_line(&mut entrada1).expect("Error en la lectura de datos");
    cant_horm = i32::from_str(&entrada1.trim()).unwrap();

    println!("\nIntroduce la cantidad de Arañas capturadas");
    let mut entrada2 = String::new();

    io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");
    cant_spiders = i32::from_str(&entrada2.trim()).unwrap();

    println!("\nIntroduce la cantidad de Cochinillas capturadas");
    let mut entrada3 = String::new();

    io::stdin().read_line(&mut entrada3).expect("Error en la lectura de datos");
    cant_cochinillas = i32::from_str(&entrada3.trim()).unwrap();

    cant_total_patas = get_num_patas_hormigas(cant_horm, NUM_PATAS_HORMIGA) 
        + get_num_patas_spiders(cant_spiders, NUM_PATAS_SPIDER) 
        + get_num_patas_cochinillas(cant_cochinillas, NUM_PATAS_COCHINILLA);

    println!("\nCantidad Total de Patas:\t{}\tpatas\n", cant_total_patas);
}

fn get_num_patas_hormigas(cant_horm: i32, num_patas: i32) -> i32 {
    return cant_horm * num_patas;
}

fn get_num_patas_spiders(cant_spiders: i32, num_patas: i32) -> i32 {
    return cant_spiders * num_patas;
}

fn get_num_patas_cochinillas(cant_cochinillas: i32, num_patas: i32) -> i32 {
    return cant_cochinillas * num_patas;
}
