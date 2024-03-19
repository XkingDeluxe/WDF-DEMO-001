//Dependencies/crates
use crate::wdf::wdf::*;

pub enum Quality {  //Quality. Good uses lighter calculations but is less accurate than Best
    GOOD,
    BEST
}

pub struct DiodePair{
    pub wdf:WDF,
    ext_imp: f32,
    q: Quality,
    is: f32, //reverse saturation current
    vt: f32, //thermal voltage
    two_vt:f32,
    one_over_vt:f32,
    imp_is: f32,
    imp_is_over_vt: f32,
    log_imp_is_over_vt: f32
}
impl DiodePair {                            //Calculations needed
    pub fn calc_impedance(&mut self){
        self.imp_is = self.ext_imp * self.is;
        self.imp_is_over_vt = self.imp_is * self.one_over_vt;
        self.log_imp_is_over_vt = self.imp_is_over_vt.log10();
    }

    pub fn incident(&mut self, x:f32) {
        self.wdf.inci_wave = x;
    }
    fn reflected_internal_good(&mut self){
        let lambda = self.wdf.inci_wave.signum();                                       //Wright omega :
        self.wdf.refl_wave = self.wdf.inci_wave + 2.0 * lambda * (self.imp_is - self.vt * wright_omega::wright_omega((self.log_imp_is_over_vt + lambda * self.wdf.inci_wave * self.one_over_vt + self.imp_is_over_vt).into()).unwrap().norm());
    }
    fn reflected_internal_best(&mut self){
        let lambda = self.wdf.inci_wave.signum();
        let lamda_a_over_vt = lambda * self.wdf.inci_wave * self.one_over_vt;
        self.wdf.refl_wave = self.wdf.inci_wave - self.two_vt * lambda * (wright_omega::wright_omega((self.log_imp_is_over_vt + lamda_a_over_vt).into()).unwrap().norm()
                             - wright_omega::wright_omega((self.log_imp_is_over_vt - lamda_a_over_vt).into()).unwrap().norm())
    }
    pub fn reflected(&mut self) -> f32{
        match self.q {
            Quality::GOOD=> self.reflected_internal_good(),
            Quality::BEST=> self.reflected_internal_best()
        }
        self.wdf.refl_wave
    }
}
impl DiodePair {
    pub fn new(_is: f32, _vt: f32, n_diodes: u8, _quality:Quality) -> DiodePair{
        let mut dp = DiodePair{
            wdf:WDF::new(),
            ext_imp: 0.0,
            q: Quality::GOOD,
            is: 0.0,
            vt: 0.0,
            two_vt:0.0,
            one_over_vt:0.0,
            imp_is: 0.0,
            imp_is_over_vt: 0.0,
            log_imp_is_over_vt: 0.0
        };
        dp.set_parameters(_is, _vt, n_diodes, _quality);
        dp
    }
    pub fn set_parameters(&mut self, _is: f32, _vt: f32, n_diodes: u8, _quality:Quality){
        self.q = _quality;
        self.is = _is;
        self.vt = _vt * n_diodes as f32;
        self.two_vt = 2.0 * self.vt;
        self.one_over_vt = 1.0 / self.vt;
        self.calc_impedance();
    }
    pub fn set_ext_imp(&mut self, _imp:f32){
        self.ext_imp = _imp;
        self.calc_impedance();
    }
    
}