//Dependencies/crates
use crate::wdf::wdf::*;

pub struct Capacitor{
    pub wdf:WDF,
    c:f32,
    sr: f32, //sample rate
    z: f32 //state
}

impl Capacitor{ //Calculations needed for capacitor
    pub fn calc_impedance(&mut self){
        self.wdf.imp = 1.0 / (2.0 * self.sr * self.c);
        self.wdf.add = 1.0 / self.wdf.imp;
    }

    pub fn incident(&mut self, x:f32) {
        self.wdf.inci_wave = x;
        self.z = self.wdf.inci_wave;
    }
    pub fn reflected(&mut self) -> f32{
        self.wdf.refl_wave = self.z;
        self.wdf.refl_wave
    }
}

impl Capacitor {
    pub fn new(_c:f32) -> Capacitor{
        let mut c = Capacitor{
            wdf:WDF::new(),
            c:_c,
            sr:48000.0,
            z:0.0
        };
        c.calc_impedance();
        c
    }
    pub fn prepare(&mut self, _sr:f32){
        self.sr = _sr;
        self.calc_impedance();
        self.reset();
    }
    pub fn set_capacitance(&mut self, _c:f32){
        self.c=_c;
        self.calc_impedance();
    }
    pub fn reset(&mut self){
        self.z = 0.0;
    }
}