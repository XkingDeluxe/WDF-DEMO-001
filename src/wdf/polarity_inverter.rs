//Dependencies/crates
use crate::wdf::{ucomponent::UnknownComp, wdf::*};
use std::any::Any;
pub struct PolarityInverter{
    pub wdf:WDF,
    pub port1: UnknownComp
}

impl PolarityInverter { //Calculations needed for Polarity inverter
    pub fn calc_impedance(&mut self){
        self.port1.get_wdf();
        self.wdf.imp = self.port1.wdf.imp;
        self.wdf.add = 1.0 / self.wdf.imp;
    }

    pub fn incident(&mut self, x: f32){
        self.port1.get_wdf();
        self.wdf.inci_wave = x;
        self.port1.incident(-x);
        
    }
    pub fn reflected(&mut self)->f32{
        self.wdf.refl_wave = -self.port1.reflected();
        self.wdf.refl_wave
    }
}

impl PolarityInverter{                                                                //To invert polairity
    pub unsafe fn new(_port1:Box<dyn Any + Send>) -> PolarityInverter{
        let mut poli = PolarityInverter{
            wdf: WDF::new(),
            port1: UnknownComp::new(_port1)
        };
        poli.get_wdfs();
        poli.calc_impedance();
        poli
    }
    fn get_wdfs(&mut self){
        self.port1.get_wdf();
    }
    fn set_wdfds(&mut self){
        self.port1.set_wdf();
    }
}