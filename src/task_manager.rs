use crate::status::Status;
use crate::task::Task;

/// Guarda las tareas en un Vec. Una tarea se identifica por su posición (índice
/// base 0) en ese Vec. Un índice solo es válido hasta la siguiente operación que
/// modifique el Vec (por ejemplo, `delete_task` desplaza los índices siguientes).
pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    // Método constructor
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    // Crea una tarea en estado `ToDo` y devuelve su índice.
    pub fn create_task(&mut self, title: String, description: String) -> usize {
        let index = self.tasks.len();
        self.tasks.push(Task::new(title, description));
        index
    }

    // Agrega una tarea ya construida y devuelve su índice (para los datos precargados)
    pub fn add_task(&mut self, task: Task) -> usize {
        let index = self.tasks.len();
        self.tasks.push(task);
        index
    }

    pub fn get_task(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }

    pub fn get_task_mut(&mut self, index: usize) -> Option<&mut Task> {
        self.tasks.get_mut(index)
    }

    // Elimina la tarea y la devuelve para imprimir su título
    // Devuelve None si el índice no existe
    pub fn delete_task(&mut self, index: usize) -> Option<Task> {
        if index < self.tasks.len() {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    // Vista para la lista: índices reales ordenados por última modificación
    // El vector real no se ordena o los índices no coincidirían
    pub fn list_view(&self) -> Vec<usize> {
        let mut view: Vec<usize> = (0..self.tasks.len()).collect();
        view.sort_by_key(|&i| std::cmp::Reverse(self.tasks[i].modified_at()));
        view
    }

    // Vista para el tablero: agrupada por estado y ordena por última modificación
    pub fn board_view(&self) -> Vec<usize> {
        let by_date = self.list_view();
        Status::ALL
            .iter()
            .flat_map(|&status| {
                by_date
                    .iter()
                    .copied()
                    .filter(move |&i| self.tasks[i].status() == status)
            })
            .collect()
    }
}
