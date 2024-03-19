//Dependencies/crates
use crate::wdf::{ucomponent::UnknownComp, wdf::*};
use std::any::Any;

pub struct WDFSeries{
    pub wdf:WDF,
    pub port1: UnknownComp,
    pub port2: UnknownComp,
    port1_reflect: f32
}
impl WDFSeries{                 //Calculations needed for series adaptor
    pub fn calc_impedance(&mut self){ 
        self.get_wdfs();
        self.wdf.imp = self.port1.wdf.imp + self.port2.wdf.imp;
        self.wdf.add = 1.0/self.wdf.imp;
        self.port1_reflect = self.port1.wdf.imp / self.wdf.imp;
    }

    pub fn incident(&mut self, x: f32){
        self.get_wdfs();
        let b1 = self.port1.wdf.refl_wave - self.port1_reflect * (x + self.port1.wdf.refl_wave + self.port2.wdf.refl_wave);
        self.port1.incident(b1);
        self.port2.incident(-(x+b1));
        self.wdf.inci_wave = x;
    }
    pub fn reflected(&mut self)->f32{
        self.get_wdfs();
        self.wdf.refl_wave = -(self.port1.reflected() + self.port2.reflected());
        self.wdf.refl_wave
    }
}

impl WDFSeries{
    pub unsafe fn new(_port1:Box<dyn Any + Send>, _port2:Box<dyn Any + Send>) -> WDFSeries { //Initialize series adaptor
        let mut wdfs = WDFSeries{
            wdf: WDF::new(),
            port1: UnknownComp::new(_port1),    //Assign component 1
            port2: UnknownComp::new(_port2),    //Assign component 2
            port1_reflect: 0.0
        };
        wdfs.get_wdfs();
        wdfs.calc_impedance();
        wdfs
    }
    pub fn _retrieve_borrow<T1: 'static, T2: 'static>(&mut self) -> (Option<&mut T1>, Option<&mut T2>){ //Borrow components back
        (self.port1.comp.downcast_mut::<T1>(), self.port2.comp.downcast_mut::<T2>())
    }
    fn get_wdfs(&mut self){
        self.port1.get_wdf();
        self.port2.get_wdf();
    }
    fn _set_wdfs(&mut self){
        self.port1.set_wdf();
        self.port2.set_wdf();
    }


}

pub struct WDFParallel{
    pub wdf:WDF,
    pub port1: UnknownComp,
    pub port2: UnknownComp,
    port1_reflect: f32,
    reflect_diff: f32
}

impl WDFParallel{                 //Calculations needed for parallel adaptor
    pub fn calc_impedance(&mut self){
        self.get_wdfs();
        self.wdf.add = self.port1.wdf.add + self.port2.wdf.add;
        self.wdf.imp = 1.0/self.wdf.add;
        self.port1_reflect = self.port1.wdf.add / self.wdf.add;
    }

    pub fn incident(&mut self, x: f32){
        self.get_wdfs();
        let b2 = self.wdf.refl_wave - self.port2.wdf.refl_wave + x;
        self.port1.incident(b2 + self.reflect_diff);
        self.port2.incident(b2);
        self.wdf.inci_wave = x;
    }
    pub fn reflected(&mut self)->f32{
        self.port1.reflected();
        self.port2.reflected();
        self.get_wdfs();
        self.reflect_diff = self.port2.wdf.refl_wave - self.port1.wdf.refl_wave;
        self.wdf.refl_wave = self.port2.wdf.refl_wave - self.port1_reflect * self.reflect_diff;
        self.wdf.refl_wave
    }
}

impl WDFParallel{
    pub unsafe fn new(_port1:Box<dyn Any + Send>, _port2:Box<dyn Any + Send>) -> WDFParallel { //Initialize parralel adaptor
        let mut wdfs = WDFParallel{
            wdf: WDF::new(),
            port1: UnknownComp::new(_port1),
            port2: UnknownComp::new(_port2),
            port1_reflect: 1.0,
            reflect_diff: 0.0
        };
        wdfs.get_wdfs();
        wdfs.calc_impedance();
        wdfs
    }
    pub fn _retrieve_borrow<T1: 'static, T2: 'static>(&mut self) -> (Option<&mut T1>, Option<&mut T2>){
        (self.port1.comp.downcast_mut::<T1>(), self.port2.comp.downcast_mut::<T2>())
    }
    fn get_wdfs(&mut self){
        self.port1.get_wdf();
        self.port2.get_wdf();
    }
    fn _set_wdfs(&mut self){
        self.port1.set_wdf();
        self.port2.set_wdf();
    }
}
