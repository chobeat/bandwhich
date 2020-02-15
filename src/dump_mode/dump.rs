
use std::sync::{Mutex, Arc};
use crate::display::Ui;
use tui::backend::Backend;


pub struct Dumper<B>
    where
        B: Backend
{

    ui: Arc<Mutex<Ui<B>>>,
}

impl<B> Dumper<B>
    where
        B: Backend
{

    pub fn new(ui: Arc<Mutex<Ui<B>>>)->Self{
        Dumper{ui}
    }
    pub fn update_state(&self){println!("update")}
    pub fn dump(&self){println!("dump")}
}