//Dependencies/crates
use crate::wdf::resistor::Resistor;
use crate::wdf::capacitor::Capacitor;
use crate::wdf::voltage_sources::IdealVoltageSource;
use crate::wdf::adaptors::*;

pub struct RCLowpass{
    s1: WDFSeries,
    vs: IdealVoltageSource
}

impl RCLowpass {
    pub fn new(_r: f32, _c: f32, _sample_rate: f32) ->RCLowpass{//Create RC Lowpass
        let r1 = Resistor::new(_r);
        let mut c1 = Capacitor::new(_c);
        c1.prepare(_sample_rate);

        RCLowpass{
            s1:unsafe{WDFSeries::new(Box::new(r1), Box::new(c1))}, //Assign R1 and C1 to adaptor. These are dropped so can't be accessed anymore
            vs: IdealVoltageSource::new() //Initialize voltage source (input)
        }
    }
    pub fn tick(&mut self, _vs: f32) -> f32{
        self.vs.set_voltage(_vs);               //Set voltagesource to input voltage
        self.vs.incident(self.s1.reflected());  //Set VS incident as S1 Reflected
        self.s1.incident(self.vs.reflected());  //Set S1 incident as VS Reflected
        self.s1.port2.comp.downcast_mut::<Capacitor>().unwrap().wdf.voltage()   //Output voltage as voltage over c1
    }
    pub fn update(&mut self, _rcr:f32, _rcc:f32){   //Update any value changes
        self.s1.port1.comp.downcast_mut::<Resistor>().unwrap().set_resistance(_rcr);
        self.s1.port2.comp.downcast_mut::<Capacitor>().unwrap().set_capacitance(_rcc);
        self.s1.calc_impedance();
    }
}