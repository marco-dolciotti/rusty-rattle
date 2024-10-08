use std::time::Duration;

pub mod controller;
pub mod event_processes;
pub mod model;
pub mod views;

#[derive(Clone, Copy)]
pub struct Config {
    view_type: ViewType,
    grid_height: usize,
    grid_width: usize,
    update_interval: Duration,
}

impl Config {
    pub fn view_type(&self) -> &ViewType {
        &self.view_type
    }

    pub fn builder() -> ConfigBuilder {
        ConfigBuilder {
            view_type: None,
            grid_height: None,
            grid_width: None,
            update_interval: None,
        }
    }
}
pub struct ConfigBuilder {
    view_type: Option<ViewType>,
    grid_height: Option<usize>,
    grid_width: Option<usize>,
    update_interval: Option<Duration>,
}

impl ConfigBuilder {
    pub fn set_view_type(mut self, view_type: ViewType) -> ConfigBuilder {
        self.view_type = Some(view_type);
        return self;
    }

    pub fn set_grid_height(mut self, grid_height: usize) -> ConfigBuilder {
        self.grid_height = Some(grid_height);
        return self;
    }

    pub fn set_grid_width(mut self, grid_width: usize) -> ConfigBuilder {
        self.grid_width = Some(grid_width);
        return self;
    }

    pub fn set_update_interval(mut self, update_interval: Duration) -> ConfigBuilder {
        self.update_interval = Some(update_interval);
        return self;
    }

    pub fn build(self) -> Config {
        Config {
            view_type: self.view_type.unwrap_or(ViewType::TUI),
            grid_height: self.grid_height.unwrap_or(10),
            grid_width: self.grid_width.unwrap_or(20),
            update_interval: self.update_interval.unwrap_or(Duration::from_millis(200)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum ViewType {
    GUI,
    TUI,
}
