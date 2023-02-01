use std::any::TypeId;

use crate::{system::BoxedDescribedSystem, widget::Widget, widget_batch::WidgetBatch};

pub struct Stack {
    pub children: Vec<Box<dyn Widget>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn with_children<B: WidgetBatch>(mut self, children: B) -> Self {
        self.children.reserve(B::CAPACITY);
        self.children.extend(children.into_iter());
        self
    }
}

impl Widget for Stack {
    fn fetch_events(&mut self, event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        for child in &mut self.children {
            let vec = child.fetch_events(event_type);
            if !vec.is_empty() {
                return vec;
            }
        }
        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
