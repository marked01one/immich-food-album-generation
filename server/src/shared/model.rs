use std::io::Result;
use tch::vision::image;
use tch::{CModule, Kind, Tensor};

pub struct Model {
    pub model: tch::CModule,
    pub labels: Vec<String>,
}

impl Model {
    pub fn new(model_file: String, labels: Vec<String>) -> Self {
        let model = CModule::load(&model_file).expect("Failed to load model!");

        Self { model, labels }
    }

    pub fn reload_model(&mut self, model_file: &str) {
        self.model = CModule::load(model_file).expect("Failed to load model!");
    }

    pub fn predict(&self, image: Tensor) -> Result<Tensor> {
        let output = self
            .model
            .forward_ts(&[image.unsqueeze(0)])
            .expect("Failed to get predictions with tensor!")
            .softmax(0, Kind::Float);

        Ok(output)
    }

    pub fn predict_from_file(&self, file: &str) -> Result<Tensor> {
        let tensor = image::load_and_resize(file, 224, 224)
            .expect("Failed to import image file")
            .to_kind(Kind::Float)
            / 255.0;

        let output = tensor
            .unsqueeze(0)
            .apply(&self.model)
            .softmax(-1, Kind::Float);

        Ok(output)
    }
}
