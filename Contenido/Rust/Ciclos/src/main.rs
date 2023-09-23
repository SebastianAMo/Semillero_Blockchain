
fn main() {
    
    /*
    Uso de if - else if - else
    */
    let x = 12;
    if x == 8 {
        println!("x es ocho!");
    } else if x % 4 == 0 {
        println!("If: X es divisible por 4!");
    } else {
        println!("If: X no es ocho y tampoco divisible por 4");
    }
    // uso de let if 
    
    let z = 6;
    let m = if z == 5 {
        10
    } else {
    15
    }; 
    println!("letIF: El valor de M es: {}", m); 

    //uso de loop y break
    
    let mut contador = 0;
    let resultado = loop {
        contador += 1;
        if contador == 10 {
            break contador * 2;
        }
    };
    println!("Loop: El Resultado es {resultado}");

    // uso de loop con etiqueta de break
    {
    let mut count = 0;
    'counting_up: loop {
        println!("Contador = {count}"); 
        let mut remaining = 10;
        loop {
            println!("Faltante = {remaining}"); 
            if remaining == 9 {
                break; 
            }
            if count == 2 {
                break 'counting_up; 
            }
            remaining -= 1;
            }
        count += 1; 
        }
        println!("Contador final = {count}"); 
    }
    // for utilizando la funciona enumerate para contar lineas
    let lineas = "hola\nmundo\nRust".lines();
    for (numero_linea, linea) in lineas.enumerate() {
    println!("{}: {}", numero_linea, linea);
}
let x = 3;
//uso del match
let number = match x {
    1 => println!("Match: uno"),
    2 => println!("Match: dos"),
    3 => println!("Match: tres"),
    4 => println!("Match: cuatro"),
    5 => println!("Match: cinco"),
    _ => println!("Match: otro número diferente de 0-5"),
};

}
