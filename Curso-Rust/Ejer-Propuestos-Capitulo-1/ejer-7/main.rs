/*  Ejer 7  Solicitar al usuario tres distancias:
                * la primera, medida en milímetros
                * la segunda, medida en centímetros
                * y la última, medida en metros
            
            Diseñar un programa que muestre la suma de las tres longitudes introducidas (medidas en
            centímetros).
*/

use std::io;
use std::str::FromStr;

fn main() {
    let dist_m: f32;
    let dist_cm: f32;
    let dist_mm: f32;
    let suma_distancias: f32;

    println!("\nIntroduce Primera medida en mm");
    let mut entrada1 = String::new();

    io::stdin().read_line(&mut entrada1).expect("Error en la lectura de datos");
    dist_mm = f32::from_str(&entrada1.trim()).unwrap();

    println!("\nIntroduce Segunda medida en cm");
    let mut entrada2 = String::new();

    io::stdin().read_line(&mut entrada2).expect("Error en la lectura de datos");
    dist_cm = f32::from_str(&entrada2.trim()).unwrap();

    println!("\nIntroduce Tercera medida en metros");
    let mut entrada3 = String::new();

    io::stdin().read_line(&mut entrada3).expect("Error en la entrada de datos");
    dist_m = f32::from_str(&entrada3.trim()).unwrap();

    /* 
    let mut sum_aux: f32 = get_milimetros_cm(dist_mm);

    sum_aux += dist_cm;

    sum_aux += get_metros_cm(dist_m);

    suma_distancias = sum_aux;
    */

    suma_distancias = get_milimetros_cm(dist_mm) + dist_cm + get_metros_cm(dist_m);
    23.6
    ;


    println!("\nSuma de la tres medida introducidas en CM:\t{}\tcm\n", suma_distancias);
}

fn get_metros_cm(dist_m: f32) -> f32 { 
    return dist_m * 100.0;
}

/*
fn get_metros_mm(dist_m: f32) -> f32 { 
    return dist_m * 1000.0;
}

fn get_centimetros_mm(dist_cm: f32) -> f32 { 
    return dist_cm * 10.0;
}
*/

fn get_milimetros_cm(dist_mm: f32) -> f32 { 
    return dist_mm / 10.0;
}

/* 
fn get_suma_distancias(dist1: f32, dist2: f32, dist3: f32) -> f32 { 
    return dist1 +  dist2 + dist3;
}
*/    
