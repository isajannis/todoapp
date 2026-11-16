use chrono::{DateTime, Duration, Local};

use crate::status::Status;

pub struct Task {
    pub title: String,
    pub description: String,
    status: Status,
    pub created_at: DateTime<Local>,
    modified_at: DateTime<Local>,
    finished_at: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(title: String, description: String) -> Self {
        let now = Local::now();
        Task {
            title,
            description,
            status: Status::ToDo,
            created_at: now,
            modified_at: now, // por defecto igual a la creación
            finished_at: None,
        }
    }

    // Construye una tarea con estado y fechas explícitos (datos de prueba).
    // `finished_at` solo se guarda si el estado es Done
    pub fn with_history(
        title: String,
        description: String,
        status: Status,
        created_at: DateTime<Local>,
        modified_at: DateTime<Local>,
        finished_at: Option<DateTime<Local>>,
    ) -> Self {
        Task {
            title,
            description,
            status,
            created_at,
            modified_at,
            finished_at: if status == Status::Done { finished_at } else { None },
        }
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn modified_at(&self) -> DateTime<Local> {
        self.modified_at
    }

    // Cambia el estado
    pub fn set_status(&mut self, new_status: Status) {
        if new_status == self.status {
            return;
        }
        let now = Local::now();
        self.status = new_status;
        self.modified_at = now;
        self.finished_at = if new_status == Status::Done {
            Some(now)
        } else {
            None
        };
    }

    // Terminada: término - creación. Cualquier otro estado: ahora - creación.
    pub fn duration(&self) -> Duration {
        let end = self.finished_at.unwrap_or_else(Local::now);
        end - self.created_at
    }
}
