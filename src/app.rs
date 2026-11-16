use chrono::{DateTime, Duration, Local};

use crate::input::{prompt, prompt_non_empty, read_option};
use crate::seed;
use crate::status::Status;
use crate::task::Task;
use crate::task_manager::TaskManager;

// Variable para precargar tareas
const PRELOAD_TASKS: bool = true;

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    List,
    Board,
}

enum Screen {
    Main,               // Menú Principal
    Tasks(ViewMode),    // Vista de Tareas: en lista o en tablero
    Detail(usize),      // Detalles de una Tarea
    Exit,               // Salir del programa
}

pub fn run() {
    println!("Bienvenido al programa To Do App en Rust!");

    let mut manager: TaskManager = TaskManager::new();
    if PRELOAD_TASKS {
        seed::preload(&mut manager);
    }
    let mut screen: Screen = Screen::Main;

    loop {
        // Control sobre lo que se le es presentado al usuario
        screen = match screen {
            Screen::Main => main_menu(&mut manager),
            Screen::Tasks(mode) => tasks_screen(&mut manager, mode),
            Screen::Detail(index) => detail_screen(&mut manager, index),
            Screen::Exit => break,
        };
    }
    println!("Hasta luego!");
}

// ---------- Pantallas ----------

fn main_menu(manager: &mut TaskManager) -> Screen {
    println!("\n¿Qué quieres hacer hoy?");
    println!("1. Crear nueva tarea");
    println!("2. Ver tareas");
    println!("3. Salir");
    match read_option("> ", 3) {
        1 => {
            create_task(manager);
            Screen::Main
        }
        2 => Screen::Tasks(ViewMode::List),
        _ => Screen::Exit,
    }
}

fn tasks_screen(manager: &mut TaskManager, mode: ViewMode) -> Screen {
    println!();
    println!("Buscando tareas guardadas...");

    let view = match mode {
        ViewMode::List => manager.list_view(),
        ViewMode::Board => manager.board_view(),
    };

    println!();
    if view.is_empty() {
        println!("No hay tareas todavía.");
    } else {
        match mode {
            ViewMode::List => print_list(manager, &view),
            ViewMode::Board => print_board(manager, &view),
        }
    }

    println!("\n¿Qué quieres hacer ahora?");
    println!("1. Crear nueva tarea");
    println!("2. Ver detalles de una tarea");
    match mode {
        ViewMode::List => println!("3. Ver en formato tablero"),
        ViewMode::Board => println!("3. Ver en formato lista"),
    }
    println!("4. Volver al menú principal");

    match read_option("> ", 4) {
        1 => {
            create_task(manager);
            Screen::Tasks(mode)
        }
        2 => {
            if view.is_empty() {
                println!("No hay tareas para ver.");
                return Screen::Tasks(mode);
            }
            let n = read_option("Número de la tarea: ", view.len());
            Screen::Detail(view[n - 1]) // n >= 1 garantizado por read_option
        }
        3 => Screen::Tasks(match mode {
            ViewMode::List => ViewMode::Board,
            ViewMode::Board => ViewMode::List,
        }),
        _ => Screen::Main,
    }
}

fn detail_screen(manager: &mut TaskManager, index: usize) -> Screen {
    let Some(task) = manager.get_task(index) else {
        println!("No existe esa tarea.");
        return Screen::Tasks(ViewMode::List);
    };

    println!();
    print_detail(task);

    println!("\n¿Qué quieres hacer ahora?");
    println!("1. Modificar un campo");
    println!("2. Eliminar tarea");
    println!("3. Volver a ver tareas");
    println!("4. Volver al menú principal");

    match read_option("> ", 4) {
        1 => {
            modify_field(manager, index);
            Screen::Detail(index)
        }
        2 => {
            if let Some(task) = manager.delete_task(index) {
                println!("La tarea \"{}\" ha sido eliminada exitosamente", task.title);
            }
            // Los índices cambiaron: no seguimos en Detail con uno viejo.
            Screen::Tasks(ViewMode::List)
        }
        3 => Screen::Tasks(ViewMode::List),
        _ => Screen::Main,
    }
}

// ---------- Acciones ----------

fn create_task(manager: &mut TaskManager) {
    let title = prompt_non_empty("Título: ");
    let description = prompt("Descripción: ");
    manager.create_task(title, description);
    println!("Tarea creada exitosamente!");
}

fn modify_field(manager: &mut TaskManager, index: usize) {
    println!("\nCampos modificables:");
    println!("1. Título");
    println!("2. Estado");
    println!("3. Descripción");
    let field = read_option("> ", 3);

    // Pedimos primero el nuevo valor y recién después tomamos el préstamo
    // mutable, para no mantenerlo abierto mientras esperamos al usuario.
    let name = match field {
        1 => {
            let value = prompt_non_empty("Nuevo título: ");
            apply(manager, index, |t| t.title = value);
            "título"
        }
        2 => {
            println!("Estados:");
            for (i, s) in Status::ALL.iter().enumerate() {
                println!("{}. {}", i + 1, s);
            }
            let new_status = Status::ALL[read_option("> ", Status::ALL.len()) - 1];
            apply(manager, index, |t| t.set_status(new_status));
            "estado"
        }
        _ => {
            let value = prompt("Nueva descripción: ");
            apply(manager, index, |t| t.description = value);
            "descripción"
        }
    };
    println!("El campo {} ha sido modificado exitosamente!", name);
}

fn apply(manager: &mut TaskManager, index: usize, f: impl FnOnce(&mut Task)) {
    if let Some(task) = manager.get_task_mut(index) {
        f(task);
    }
}

// ---------- Presentación ----------

// Vista en lista
fn print_list(manager: &TaskManager, view: &[usize]) {
    for (pos, &i) in view.iter().enumerate() {
        if let Some(task) = manager.get_task(i) {
            print_summary(pos + 1, task);
        }
    }
}

// Vista separada por estados
fn print_board(manager: &TaskManager, view: &[usize]) {
    for status in Status::ALL {
        println!("============ {} ============", status);
        let mut empty = true;
        for (pos, &i) in view.iter().enumerate() {
            if let Some(task) = manager.get_task(i) {
                if task.status() == status {
                    print_summary(pos + 1, task);
                    empty = false;
                }
            }
        }
        if empty {
            println!("  (vacío)");
        }
    }
}

// Información de la tarea resumida
fn print_summary(number: usize, task: &Task) {
    println!("Tarea número: {}", number);
    println!("Título: {}", task.title);
    println!("Estado: {}", task.status());
    println!("Última actualización: {}", fmt_date(task.modified_at()));
    println!();
}

// Detalles de la tarea
fn print_detail(task: &Task) {
    println!("Título: {}", task.title);
    println!("Descripción: {}", task.description);
    println!("Estado: {}", task.status());
    println!("Creada: {}", fmt_date(task.created_at));
    println!("Última actualización: {}", fmt_date(task.modified_at()));
    println!("Duración: {}", fmt_duration(task.duration()));
}

fn fmt_date(date: DateTime<Local>) -> String {
    date.format("%d/%m/%Y %H:%M").to_string()
}

fn fmt_duration(d: Duration) -> String {
    let minutes = d.num_minutes().max(0);
    let (days, hours, mins) = (minutes / 1440, (minutes % 1440) / 60, minutes % 60);
    format!("{}d {}h {}m", days, hours, mins)
}
