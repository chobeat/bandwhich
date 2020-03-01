
use std::sync::{Mutex, Arc};
use crate::display::Ui;
use tui::backend::Backend;
use std::fs::File;
use std::io::Write;
use std::error::Error;


pub struct Dumper<B>
    where
        B: Backend + std::marker::Send
{

    ui: Arc<Mutex<Ui<B>>>,
    file: File
}


impl<B> Dumper<B>
    where
        B: Backend + std::marker::Send
{

    pub fn new(ui: Arc<Mutex<Ui<B>>>)->Self{
        let path = "/tmp/test.dump";
        let file = match File::create(path) {
            Err(why) => panic!("couldn't open dump file: {}\n{}",
                               path, why.description()),
            Ok(file) => file,
        };
        Dumper{ui, file}
    }
    pub fn update_state(&self){println!("update")}
    pub fn dump(&self) {
        let mut ui = self.ui.lock().unwrap();
        let mut write_to_file: Box<dyn FnMut(String) + Send> =
            Box::new({
                         move |output: String| {
                             let mut file = &self.file;
                             match file.write_all(output.as_bytes())   {

                                 Err(why) => panic!("Couldn't dump to file. \n{}",
                                                    why.description()),
                                 Ok(()) => (),

                             }
                         }
                     });
        ui.output_text(&mut write_to_file);
    }
}
