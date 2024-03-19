//Dependencies/crates
use crate::wdf::resistor::Resistor;
use crate::wdf::capacitor::Capacitor;
use crate::wdf::voltage_sources::ResistiveVoltageSource;
use crate::wdf::adaptors::*;
use crate::wdf::diode::*;

pub struct DiodeClipper{
    p1: WDFParallel,
    dp: DiodePair
}

impl DiodeClipper {
    pub fn new(_r: f32, _c: f32, _d_is:f32, _d_vt:f32, _sample_rate: f32) ->DiodeClipper{ //Create Diodeclipper
        let r1 = Resistor::new(_r);
        let vs = ResistiveVoltageSource::new(1.0e-9);
        let s1 = unsafe{WDFSeries::new(Box::new(vs), Box::new(r1))}; //VS and R1 are dropped so can't be accessed anymore
        let mut c1 = Capacitor::new(_c);
        c1.prepare(_sample_rate);               //Assign samplerate to capacitor
        DiodeClipper{
            p1:unsafe{WDFParallel::new(Box::new(s1), Box::new(c1))}, //S1 and C1 are dropped so can't be accessed anymore
            dp: DiodePair::new(_d_is, _d_vt, 1, Quality::BEST), //Initialize diodepair
        }
    }
    pub fn tick(&mut self, _vs: f32) -> f32{    //Tick to calculate input to output
        self.p1.port1.comp.downcast_mut::<WDFSeries>().unwrap().port1.comp.downcast_mut::<ResistiveVoltageSource>().unwrap().set_voltage(_vs); //Set voltagesource to inputvoltage
        self.dp.incident(self.p1.reflected());                                                     //Set diodepair indicent to p1 reflected
        let vout = self.p1.port2.comp.downcast_mut::<Capacitor>().unwrap().wdf.voltage();     //Output voltage as voltage over c1
        self.dp.set_ext_imp(self.p1.wdf.imp.clone());                                              //Update any impedance change
        self.p1.incident(self.dp.reflected());                                                     //Set p1 indicent to diodepair reflected
        vout
    }
    pub fn update(&mut self, _dcr:f32, _dcc:f32){//Update any value changes
        let s1 = self.p1.port1.comp.downcast_mut::<WDFSeries>().unwrap();
        s1.port2.comp.downcast_mut::<Resistor>().unwrap().set_resistance(_dcr);
        self.p1.port2.comp.downcast_mut::<Capacitor>().unwrap().set_capacitance(_dcc);
        s1.calc_impedance();
        self.p1.calc_impedance();
    }
}