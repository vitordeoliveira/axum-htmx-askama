#[derive(Debug)]
struct AppState {
    todos: Mutex<Vec<Option<Todo>>>,
}

#[derive(Deserialize, Debug, Clone)]
struct Todo {
    id: u16,
    value: String,
    active: bool,
}

// impl Todo {
//     fn new(id: u16, value: String, active: bool) -> Self {
//         Self { id, value, active }
//     }
// }

// Model Controller
// Clone here just clone the ARC not the Vector
#[derive(Clone)]
pub struct ModelController {
    todos_store: Arc<Mutex<Vec<Option<Todo>>>>,
}
