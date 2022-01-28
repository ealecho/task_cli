pub struct CreateTaskInteractor {}

impl CreateTaskInteractor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn execute(&self, _data: String) {
        println!("CreateTaskInteractor.execute: {}", _data)
    }
}