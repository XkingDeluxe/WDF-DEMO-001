//Dependencies/crates
use crate::wdf::wdf::*;

pub struct IdealVoltageSource{
    pub wdf:WDF,
    pub vs: f32
}

impl IdealVoltageSource {  //Calculations needed for Ideal voltage source
    pub fn incident(&mut self, x:f32) {
        self.wdf.inci_wave = x;
    }
    pub fn reflected(&mut self) -> f32{
        self.wdf.refl_wave = -self.wdf.inci_wave + 2.0  * self.vs;
        self.wdf.refl_wave
    }
}

impl IdealVoltageSource {
    pub fn new() -> IdealVoltageSource{
        IdealVoltageSource{
            wdf: WDF::new(),
            vs: 0.0
        }
    }
    pub fn set_voltage(&mut self, voltage: f32){
        self.vs = voltage;
    }
}

pub struct ResistiveVoltageSource{
    pub wdf:WDF,
    vs: f32,
    r: f32
}

impl ResistiveVoltageSource { //Calculations needed for Resistive voltage source
    pub fn calc_impedance(&mut self){
        self.wdf.imp = self.r;
        self.wdf.add = 1.0 / self.wdf.imp;
    }

    pub fn incident(&mut self, x:f32) {
        self.wdf.inci_wave = x;
    }
    pub fn reflected(&mut self) -> f32{
        self.wdf.refl_wave = self.vs;
        self.wdf.refl_wave
    }
}

impl ResistiveVoltageSource {
    pub fn new(_r: f32) -> ResistiveVoltageSource{
        let mut resv = ResistiveVoltageSource{
            wdf: WDF::new(),
            vs: 0.0, 
            r: _r
        };
        resv.calc_impedance();
        resv
        
    }
    pub fn set_voltage(&mut self, voltage: f32){
        self.vs = voltage;
    }
    pub fn _set_resistance(&mut self, _r: f32){
        self.r = _r;
        self.calc_impedance();
    }
}