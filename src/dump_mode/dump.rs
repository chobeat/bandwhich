use crate::display::UIState;


pub struct Dumper
{
    state: UIState,
}

impl Dumper{
    pub fn new(state:UIState)->Dumper{
        Dumper{state}
    }
    pub fn update_state(self, ui_state:UIState){}
    pub fn dump(self){}
}