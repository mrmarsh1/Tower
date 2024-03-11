use std::any::Any;

pub struct Services {
    services: Vec<Box<dyn Any>>,
}

impl Services {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn add_service<T: 'static>(&mut self, item: T) {
        self.services.push(Box::new(item));
    }

    pub fn resolve<T: 'static>(&self) -> Option<&T> {
        self.services
            .iter()
            .find_map(|item| item.downcast_ref::<T>())
    }
}
