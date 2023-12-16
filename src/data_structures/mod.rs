use std::rc::Rc;

pub mod files;
pub mod session;
pub trait Response {}

pub trait Request {
    fn new(parameters: std::collections::HashMap<&str, Box<Rc<dyn std::any::Any>>>) -> Self;
    fn send(&self, url: &str) -> Box<dyn Response>;
}
