pub struct Model {
    pub model_file: String,
    pub model: tch::CModule,
}

impl Model {
    pub fn new(model_file: String) -> Self {
        Self {
            model_file: model_file.clone(),
            model: tch::CModule::load(model_file).expect("Failed to load model!"),
        }
    }

    pub fn reload_model(&mut self, model_file: String) {
        self.model = tch::CModule::load(model_file).expect("Failed to load model!");
    }

    pub fn predict(&self, input: tch::Tensor) -> tch::Tensor {
        self.model
            .forward_ts(&[input])
            .expect("Failed to run model!")
    }
}
