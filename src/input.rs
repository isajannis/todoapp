use std::io::{self, Write};

/// Lee una línea de stdin y la devuelve sin espacios ni salto de línea.
pub fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("No se pudo leer la entrada");
    buffer.trim().to_string()
}

/// Muestra `msg` (sin salto de línea) y lee la respuesta.
pub fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().expect("No se pudo escribir en pantalla");
    read_line()
}

/// Igual que `prompt`, pero repite hasta que la respuesta no esté vacía.
pub fn prompt_non_empty(msg: &str) -> String {
    loop {
        let input = prompt(msg);
        if !input.is_empty() {
            return input;
        }
        println!("Este campo no puede estar vacío.");
    }
}

/// Pide un número entre 1 y `max` (inclusive) y repite hasta que sea válido.
/// `max` debe ser >= 1.
pub fn read_option(msg: &str, max: usize) -> usize {
    loop {
        match prompt(msg).parse::<usize>() {
            Ok(n) if (1..=max).contains(&n) => return n,
            Ok(_) => println!("Elige un número entre 1 y {}.", max),
            Err(_) => println!("Eso no es un número válido."),
        }
    }
}
