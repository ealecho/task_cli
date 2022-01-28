use usecases::interactors::tasks::CreateTaskInteractor;

pub struct TaskController {
    create_in: CreateTaskInteractor,
}

impl TaskController {
    pub fn new(
        create_in: CreateTaskInteractor,
        // create_output_port: &CreateResourceOutputPort,
    ) -> TaskController {
        return TaskController {
            create_in,
        // create_output_port,
        };
    }

    pub fn create(&self, data: String) {
        self.create_in.execute(data);
    }
}