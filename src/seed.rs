use chrono::{Duration, Local};

use crate::status::Status;
use crate::task::Task;
use crate::task_manager::TaskManager;

// Carga tareas con variabilidad en atributos para probar el programa.
// Fechas relativas con el tiempo actual, así siempre se ven recientes.
pub fn preload(manager: &mut TaskManager) {
    let now = Local::now();

    let samples = [
        (
            "Aprender Rust",
            "A través de Rustlings",
            Status::Doing,
            Duration::days(5),
            Duration::days(2),
            None,
        ),
        (
            "Ir al supermercado",
            "Pan de completo, paltas, tomates, vienesas",
            Status::ToDo,
            Duration::hours(3),
            Duration::hours(3),
            None,
        ),
        (
            "Hacer galletas con chispas de chocolate",
            "Dejar 5 para llevar al día siguiente",
            Status::Done,
            Duration::days(10),
            Duration::days(4),
            Some(Duration::days(4)),
        ),
        (
            "Sacar la basura",
            "Pasa los lunes, miércoles y viernes",
            Status::ToDo,
            Duration::days(1),
            Duration::days(1),
            None,
        ),
        (
            "Avanzar con el informe del E",
            "Se entrega el 30 de noviembre",
            Status::Doing,
            Duration::hours(30),
            Duration::hours(6),
            None,
        ),
        (
            "Pagar cuentas",
            "Luz, agua e internet",
            Status::Done,
            Duration::days(3),
            Duration::minutes(45),
            Some(Duration::minutes(45)),
        ),
    ];

    for (title, description, status, created_ago, modified_ago, finished_ago) in samples {
        manager.add_task(Task::with_history(
            title.to_string(),
            description.to_string(),
            status,
            now - created_ago,
            now - modified_ago,
            finished_ago.map(|ago| now - ago),
        ));
    }
}
