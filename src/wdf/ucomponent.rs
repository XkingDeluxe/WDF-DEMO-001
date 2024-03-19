//Dependencies/crates
use crate::wdf::{adaptors::*, capacitor::Capacitor, resistor::Resistor, voltage_sources::*, wdf::*};
use std::any::Any;

pub struct UnknownComp{                     //Body for adaptors when component is not known
    pub comp: Box<dyn Any + Send>,
    //comp_type: TypeId,
    pub wdf: WDF
}
impl UnknownComp {
    pub fn new(_comp: Box<dyn Any + Send>) -> UnknownComp{ //Initialize unknown component
        let uc = UnknownComp{
            comp:_comp,
            //comp_type: TypeId::of::<TypeId>(),
            wdf: WDF::new()                     
        };
        //uc.get_type();
        uc
    }
    /*fn get_type(&mut self){   //Get component type
        if let Some(component) = self.comp.downcast_mut::<Resistor>(){
            self.comp_type = TypeId::of::<Resistor>();
        }else
        if let Some(component) = self.comp.downcast_mut::<Capacitor>(){
            self.comp_type = TypeId::of::<Capacitor>();
        }else
        if let Some(component) = self.comp.downcast_mut::<WDFSeries>(){
            self.comp_type = TypeId::of::<WDFSeries>();
        }else
        if let Some(component) = self.comp.downcast_mut::<WDFParallel>(){
            self.comp_type = TypeId::of::<WDFParallel>();
        }else
        if let Some(component) = self.comp.downcast_mut::<ResistiveVoltageSource>(){
            self.comp_type = TypeId::of::<WDFSeries>();
        }else{
            panic!("Something went wrong with receiving data from component 1 on series adaptor");
        }
    }*/
}
impl CompFN for UnknownComp {   //use common component functions trait to make things easier
    fn set_resistance(&mut self, _r:f32) {
        if let Some(component) = self.comp.downcast_mut::<Resistor>(){
            component.set_resistance(_r);
        }
    }
    fn get_wdf(&mut self){                                                                 //Get wdf 
        if let Some(component) = self.comp.downcast_mut::<Resistor>(){      //Check what component the unknown component is
            self.wdf = component.wdf;                                                      //Due to rust's limitations, this is what we'll have to work with for now
        }else
        if let Some(component) = self.comp.downcast_mut::<Capacitor>(){
            self.wdf = component.wdf;
        }else
        if let Some(component) = self.comp.downcast_mut::<WDFSeries>(){
            self.wdf = component.wdf;
        }else
        if let Some(component) = self.comp.downcast_mut::<WDFParallel>(){
            self.wdf = component.wdf;
        }else
        if let Some(component) = self.comp.downcast_mut::<ResistiveVoltageSource>(){
            self.wdf = component.wdf;
        }else{
            panic!("Something went wrong with receiving data from component 1 on series adaptor");
        }
    }
    fn set_wdf(&mut self){                                                                  
        if let Some(component) = self.comp.downcast_mut::<Resistor>(){
            component.wdf = self.wdf;
        }else
        if let Some(component) = self.comp.downcast_mut::<Capacitor>(){
            component.wdf = self.wdf;
        }else
        if let Some(component) = self.comp.downcast_mut::<WDFSeries>(){
            component.wdf = self.wdf;
        }else
        if let Some(component) = self.comp.downcast_mut::<WDFParallel>(){
            component.wdf = self.wdf;
        }else
        if let Some(component) = self.comp.downcast_mut::<ResistiveVoltageSource>(){
            component.wdf = self.wdf;
        }else{
            panic!("Something went wrong with receiving data from component 1 on series adaptor");
        }
    }
    fn calc_impedance(&mut self) {                                                              
        if let Some(component) = self.comp.downcast_mut::<Resistor>(){
            component.calc_impedance();
        }else
        if let Some(component) = self.comp.downcast_mut::<Capacitor>(){
            component.calc_impedance();
        }else
        if let Some(component) = self.comp.downcast_mut::<WDFSeries>(){
            component.calc_impedance();
        }else
        if let Some(component) = self.comp.downcast_mut::<WDFParallel>(){
            component.calc_impedance();
        }else
        if let Some(component) = self.comp.downcast_mut::<ResistiveVoltageSource>(){
            component.calc_impedance();
        }else{
            panic!("Something went wrong with receiving data from component 1 on series adaptor");
        }
    }
    fn incident(&mut self, x: f32){
        if let Some(component) = self.comp.downcast_mut::<Resistor>(){
            component.incident(x);
        }else
        if let Some(component) = self.comp.downcast_mut::<Capacitor>(){
            component.incident(x);
        }else
        if let Some(component) = self.comp.downcast_mut::<WDFSeries>(){
            component.incident(x);
        }else
        if let Some(component) = self.comp.downcast_mut::<WDFParallel>(){
            component.incident(x);
        }else
        if let Some(component) = self.comp.downcast_mut::<ResistiveVoltageSource>(){
            component.incident(x);
        }else{
            panic!("Something went wrong with receiving data from component 1 on series adaptor");
        }
    }
    fn reflected(&mut self)->f32 {
        if let Some(component) = self.comp.downcast_mut::<Resistor>(){
            
            component.reflected()
        }else
        if let Some(component) = self.comp.downcast_mut::<Capacitor>(){
            component.reflected()
        }else
        if let Some(component) = self.comp.downcast_mut::<WDFSeries>(){
            component.reflected()
        }else
        if let Some(component) = self.comp.downcast_mut::<WDFParallel>(){
            component.reflected()
        }else
        if let Some(component) = self.comp.downcast_mut::<ResistiveVoltageSource>(){
            component.reflected()
        }else{
            panic!("Something went wrong with receiving data from component 1 on series adaptor");
        }
    }
}