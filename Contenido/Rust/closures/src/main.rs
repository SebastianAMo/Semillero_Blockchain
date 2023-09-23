fn main() {
    
    
        // closure que imprime un texto
        let print_text = || println!("Hello, World!");
        
        print_text(); 
    
    
    let suma_uno = |x: i32| x + 1; println!("Resultado es: {}", suma_uno(12));

    // closures multi-linea
    let squared_sum = |x: i32, y: i32| {
    
        // realiza la suma de los dos parametros
        let mut sum: i32 = x + y;
        
        // obtiene la raiz cuadrada
        let mut result: i32 = sum * sum;
        
        return result;
    };
    
    // llamado al closure
    let result = squared_sum(5, 3);
    
    println!("Resultado: raiz cuadrada de la suma = {}", result);

}
