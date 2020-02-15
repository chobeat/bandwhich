use crate::display::UIState;


pub struct Dumper<'a>
{
    state: &'a UIState,
}

impl<'a> Dumper<'a>{
    pub fn new(state: &'a UIState)->Self{
        Dumper{state}
    }
    pub fn update_state(self, ui_state:UIState){println!("update")}
    pub fn dump(self){println!("dump")}
}