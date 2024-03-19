pub trait CompFN{           //Common functions
    fn get_wdf(&mut self);
    fn set_wdf(&mut self);
    fn calc_impedance(&mut self);
    fn incident(&mut self, x:f32);
    fn reflected(&mut self)->f32;
    fn set_resistance(&mut self, _r:f32);
}
#[derive(Clone, Copy)]
pub struct WDF{ //Common data body
    pub inci_wave: f32,//incident wave
    pub refl_wave: f32,//reflected wave
    pub imp: f32,  //impedance
    pub add: f32  //addmitance

}

impl WDF{
    pub fn new() -> WDF{
        WDF{
            inci_wave: 0.0,
            refl_wave: 0.0,
            imp: 1.0e-9,
            add: 1.0
        }
    }
    pub fn voltage(&self) -> f32{
        (self.inci_wave + self.refl_wave) / 2.0
    }

    pub fn _current(&self) -> f32{
        (self.inci_wave - self.refl_wave) / (2.0 * self.imp)
    }
}