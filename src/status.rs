use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    ToDo,
    Doing,
    Done,
}

impl Status {
    /// Todos los estados, en el orden en que se muestran (menús y tablero).
    pub const ALL: [Status; 3] = [Status::ToDo, Status::Doing, Status::Done];
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let texto: &str = match self {
            Status::ToDo => "Sin iniciar",
            Status::Doing => "En proceso",
            Status::Done => "Terminado",
        };
        write!(f, "{}", texto)
    }
}
