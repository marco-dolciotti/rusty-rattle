use crate::model::Model;

pub struct Controller {
    model: Model,
}

impl Controller {
    pub fn new(model: Model) -> Self {
        Controller { model }
    }
}
