//Dependencies/crates
use crate::diodeclipper::*;
use crate::rclowpass::*;


#[derive(Debug, Copy, Clone)] //Copy trait for moving around
pub struct CircuitInfo{ //Circuit info body
    pub rc_res: f32,
    pub rc_cap: f32,
    pub diode_res: f32,
    pub diode_cap: f32,
    pub voltage_level: f32,
    pub filter1_on: bool,
    pub filter2_on: bool
}
impl CircuitInfo {  
    pub fn new( //Initialize circuitinfo body
        _rc_res: f32,
        _rc_cap: f32,
        _diode_res: f32,
        _diode_cap: f32,
        _voltage_level: f32,
        _filter1_on: bool,
        _filter2_on: bool
    ) ->CircuitInfo{
        CircuitInfo{
            rc_res: _rc_res,
            rc_cap: _rc_cap,
            diode_res: _diode_res,
            diode_cap: _diode_cap,
            voltage_level: _voltage_level,
            filter1_on: _filter1_on,
            filter2_on: _filter2_on
        }
    }
}
pub struct CircuitHandler{              //Circuit handler body for data processing / calculation handling
    rc_lowpass: Option<RCLowpass>,  //RCLowpass body as option
    diode_clipper: Option<DiodeClipper>, //Diodeclipper body as option
    up_down_voltage: f32
}
impl CircuitHandler {
   pub fn new(rcl: Option<RCLowpass>, dc: Option<DiodeClipper>, upv:f32) -> CircuitHandler{ //Create CircuitHandler
    CircuitHandler{
        rc_lowpass: rcl,
        diode_clipper: dc,
        up_down_voltage:upv
    }
   } 
   pub fn tick(&mut self, _vs:f32, _filter1_on:bool, _filter2_on:bool) -> f32{  //Tick to create from input output. 
     let vclipper = match self.diode_clipper.as_mut() { //Check if diodeclipper is enabled
         Some(dc) => if _filter1_on {dc.tick(_vs*self.up_down_voltage)} else {_vs*self.up_down_voltage},
         None => _vs*self.up_down_voltage
     };
     let vlowpass = match self.rc_lowpass.as_mut(){ //Check if rclowpass is enabled
         Some(rc) => if _filter2_on {rc.tick(vclipper)/self.up_down_voltage} else{vclipper/self.up_down_voltage},
         None => vclipper/self.up_down_voltage
     };
     vlowpass //Return output voltage
   }

   pub fn update(&mut self, _new_circuit: CircuitInfo){ //Update voltagelevel, resistance and capacitance values
        self.up_down_voltage = _new_circuit.voltage_level;
        match self.diode_clipper.as_mut(){
            Some(clipper) => clipper.update(_new_circuit.diode_res, _new_circuit.diode_cap),
            None => ()
        }
        match self.rc_lowpass.as_mut(){
            Some(lowpass) => lowpass.update(_new_circuit.rc_res, _new_circuit.rc_cap),
            None => ()
        }
   }
}