use std::any::TypeId;

use crate::{event::Event, resource::Resource, resources::Resources, widget::Widget};

pub struct App {
    pub(crate) resources: Resources,
    widget: Box<dyn Widget>,
}

impl App {
    pub fn new<W: Widget + 'static>(widget: W) -> Self {
        Self {
            resources: Resources::new(),
            widget: Box::new(widget),
        }
    }

    pub fn insert_resource<R: Resource + 'static>(&mut self, value: R) {
        self.resources.insert_resource(value);
    }

    pub fn get_resource<R: Resource + 'static>(&self) -> Option<&R> {
        self.resources
            .get::<R>()
            .map(|raw| unsafe { &*raw.cast::<R>() })
    }

    pub fn get_resource_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        self.resources
            .get::<R>()
            .map(|raw| unsafe { &mut *raw.cast::<R>() })
    }

    pub fn dispatch_event<E: Event + 'static>(&mut self, _event: E) {
        // TODO: use event
        let mut systems = self.widget.fetch_events(TypeId::of::<E>());
        for sys in &mut systems {
            sys.lock().unwrap().run(self);
        }
    }
}
