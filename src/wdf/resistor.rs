//Dependencies/crates
use crate::wdf::wdf::*;
pub struct Resistor{
    pub wdf:WDF,
    pub r:f32
}
impl Resistor {                         //Calculations needed for resistor
    pub fn calc_impedance(&mut self){
        self.wdf.imp = self.r;
        self.wdf.add = 1.0 / self.wdf.imp;
    }

    pub fn incident(&mut self, x:f32) {
        self.wdf.inci_wave = x;
    }
    pub fn reflected(&mut self) -> f32{
        self.wdf.refl_wave = 0.0;
        self.wdf.refl_wave
    }
}
impl Resistor {
    pub fn new(_r:f32) -> Resistor{
        let mut r = Resistor{
            wdf:WDF::new(),
            r:_r
        };
        r.calc_impedance();
        r
    }
    pub fn set_resistance(&mut self, _r:f32){
        self.r=_r;
        self.calc_impedance();
    }
    
}