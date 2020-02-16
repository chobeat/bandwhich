
use std::sync::{Mutex, Arc};
use crate::display::Ui;
use tui::backend::Backend;
use std::fs::File;
use std::io::Write;


pub struct Dumper<B>
    where
        B: Backend
{

    ui: Arc<Mutex<Ui<B>>>,
    file: &mut File
}


impl<B> Dumper<B>
    where
        B: Backend
{

    pub fn new(ui: Arc<Mutex<Ui<B>>>)->Self{

        let mut file = File::create("/tmp").unwrap();
        Dumper{ui,file}
    }
    pub fn update_state(&self){println!("update")}
    pub fn dump(&self) {
        let mut ui = self.ui.lock().unwrap();
        let mut write_to_file: Box<dyn FnMut(String) + Send> =
            Box::new({
                         move |output: String| {
                             self.file.write_all(output.as_bytes());
                         }
                     });
        ui.output_text(&mut write_to_file);
    }
}

fn create_write_to_file(file:&File){

}