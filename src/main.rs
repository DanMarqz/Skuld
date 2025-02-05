mod ui;

use std::{thread, time::Duration};
use std::io;
use std::io::Write;
use colored::*;

/// Función principal del programa.
/// - Imprime el título del Pomodoro Timer con formato de color.
/// - Solicita al usuario el tiempo de trabajo, descanso y número de ciclos mediante la función `set_timer`.
/// - Inicia el ciclo de trabajo llamando a `work_cycle` con los valores ingresados.
/// - Al final, muestra un mensaje indicando que los ciclos han finalizado.
fn main() {
    // println!("{}", "~~~ Skuld | Pomodoro Timer ~~~".magenta());
    //
    //
    // let work_minutes = set_timer("Enter work time in minutes: ");
    // let rest_minutes = set_timer("Enter rest time in minutes: ");
    // let cycles_count = set_timer("Enter number of cycles: ");
    //
    // work_cycle(work_minutes, rest_minutes, cycles_count);
    //
    // println!("{}", "You finished your work cycle! Take a break! 🪓".red().bold());
    //
    // // Esperar que el usuario presione Enter para cerrar
    // println!("Press Enter to exit...");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();

    ui::run_window();
}

/// Función que ejecuta los ciclos de trabajo y descanso.
/// - **Parámetros:**
///   - `work_minutes`: Duración del tiempo de trabajo (en minutos).
///   - `rest_minutes`: Duración del tiempo de descanso (en minutos).
///   - `cycles`: Número total de ciclos a ejecutar.
/// - Realiza un bucle desde 1 hasta el número de ciclos (`cycles`).
/// - En cada ciclo:
///   - Muestra el inicio del ciclo actual.
///   - Llama a `countdown` para iniciar la cuenta regresiva del tiempo de trabajo.
///   - Si no es el último ciclo, llama a `countdown` para el tiempo de descanso.

fn work_cycle(work_minutes: u64, rest_minutes:u64, cycles:u64) {
    for cycle in 1..=cycles {
        println!("{}", format!("➡️ Cycle {} of {} started", cycle, cycles).yellow().bold());

        println!("{}", format!("🕒 Work for {} minutes", work_minutes).blue().italic());
        countdown(work_minutes);

        if cycle != cycles {
            println!("{}", "🍵 Take a break! ".green().bold());
            countdown(rest_minutes);
        }
    }
}

/// Función que realiza una cuenta regresiva en la terminal.
/// - **Parámetro:**
///   - `minutes`: Duración en minutos para la cuenta regresiva.
/// - Calcula los segundos totales y realiza un bucle inverso.
/// - Muestra el tiempo restante en formato `MM:SS` actualizando la misma línea en la terminal.
/// - Utiliza `thread::sleep` para pausar un segundo entre cada iteración.
/// - Al final, imprime una línea en blanco para limpiar la salida.
fn countdown(minutes: u64) {
    let seconds = minutes * 60;

    for remaining in (0..seconds).rev() {
        let minutes = remaining / 60;
        let seconds = remaining % 60;

        print!("\rRemaining time... {:02}:{:02}", minutes, seconds);
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_secs(1));
    }

    println!();
}

/// Función que solicita al usuario ingresar un valor válido (mayor que 0).
/// - **Parámetro:**
///   - `message`: Mensaje que se muestra al usuario solicitando la entrada.
/// - Solicita continuamente una entrada hasta que el usuario ingrese un número válido.
/// - Valida que el número ingresado sea mayor a 0.
/// - Devuelve el valor ingresado como un `u64`.
fn set_timer(message: &str) -> u64{
    loop {
        println!("{}", message);

        let mut entry = String::new();
        io::stdin().read_line(&mut entry).unwrap();

        match entry.trim().parse::<u64>() {
            Ok(value) if value > 0 => return value,
            _ => println!("{}", "Please enter a number greater than 0".red().bold()),
        }
    }
}