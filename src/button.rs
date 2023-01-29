use crate::{
    app::App,
    event::Event,
    widget::{Widget, WidgetId},
};

pub struct Button {
    pub on_click: Event,
    id: WidgetId,
}

impl Button {
    pub fn new(app: &mut App) -> &mut Self {
        app.new_widget(|id| Self {
            on_click: Event::new(),
            id,
        })
    }
}

impl Widget for Button {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
