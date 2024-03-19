//Dependencies/crates
use eframe::egui;
use rclowpass::RCLowpass;
use diodeclipper::DiodeClipper;
use crate::streamhandler::*;
use circuithandler::*;

//External files that the compiler needs to acknowledge
mod streamhandler;
mod circuithandler;
mod wdf;
mod rclowpass;
mod diodeclipper;

fn main() {
    let options = eframe::NativeOptions { //App settings
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(//App builder
        "Rust Wave Digital Filter",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<WDFApp>::default()
        }),
    ).expect("Failed to run Rust Wave Digital Filter");
}

struct WDFApp { //App body
    sender: Option<crossbeam_channel::Sender<CircuitInfo>>, //Thread communication for async communication
    stream_handler: Option<StreamHandler>, //Streamhandler body
    sh_stopped: bool,
    file_name: String,
    pub circuit_info: CircuitInfo,  //Info about the circuit
    
}
impl Default for WDFApp { //Default trait extended for WDFApp
    fn default() -> WDFApp { //Default constructor
        WDFApp {
            sender: None,
            stream_handler:None,
            sh_stopped: false,
            file_name: String::from("<Select File>"),
            circuit_info: CircuitInfo::new(
                2000.0,
                3.0e-9,
                470.0,
                47.0e-9,
                1.0,
                false,
                false
            )
            
        }
    }
}

impl WDFApp { //Implementation for WDFApp

    fn send_circuit(&mut self){//Async thread send to let know and send any changes
        self.sender.as_mut().unwrap().clone().send(self.circuit_info).expect("send failed");
    }
    fn setup_from_sel(&mut self){ //Circuit + streamhandler setup
        let rc = RCLowpass::new(self.circuit_info.rc_res, self.circuit_info.rc_cap, 44100.0);
        let dc = DiodeClipper::new(self.circuit_info.diode_res, self.circuit_info.diode_cap, 2.52e-9, 25.85e-3, 44100.0);
        let circuit = CircuitHandler::new(Some(rc), Some(dc), self.circuit_info.voltage_level);
        let fh = FileHandler::new(self.file_name.as_str(), circuit);
        self.stream_handler = Some(StreamHandler::new(&mut self.sender, fh));
        self.send_circuit();
    }
}

impl eframe::App for WDFApp { //App body extended for WDFAPP
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //Updated every frame
        egui::CentralPanel::default().show(ctx, |ui| { //ui handler
            ui.heading("Rust Wave Digital filter");
            ui.horizontal(|ui| {                        //File selector
                let name_label = ui.label("Selected file: ");
                if ui.button(self.file_name.to_string()).labelled_by(name_label.id).clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("WAV Audio", &["wav"]).pick_file(){
                        self.file_name = path.display().to_string();
                        self.setup_from_sel();
                    }
                }
            });
            match self.stream_handler.as_mut() {    //Check if file has been selected, if so render following objects:
                Some(sh) => {
                    ui.horizontal(|ui| {
                        if ui.button("Play").clicked() {
                            sh.play();
                        }
                        if ui.button("Pause").clicked() {
                            sh.pause();
                        }
                        if ui.button("Stop").clicked() {
                            sh.stop();
                            self.sh_stopped = true;
                            self.file_name = String::from("<Select File>")
                        }
                    });
                    ui.label("Filters");
                    ui.horizontal(|ui| {    //Check boxes
                        if ui.checkbox(&mut self.circuit_info.filter1_on, "Diode clipper").clicked() {
                            self.send_circuit();
                        }
                        if ui.checkbox(&mut self.circuit_info.filter2_on, "RC lowpass").clicked() {
                            self.send_circuit();
                        }
                    });
                    //-----------------------------------------------------------------------------------Component value sliders
                    let vl = ui.add(egui::Slider::new(&mut self.circuit_info.voltage_level, 0.5..=4.0).text("Volt level"));
                    let rcr = ui.add(egui::Slider::new(&mut self.circuit_info.rc_res, 1.0..=2000.0).text("RC R"));
                    let rcc = ui.add(egui::Slider::new(&mut self.circuit_info.rc_cap, 1.0e-10..=1.0e-7).text("RC C"));
                    let dcr = ui.add(egui::Slider::new(&mut self.circuit_info.diode_res, 1.0..=2000.0).text("DC R"));
                    let dcc = ui.add(egui::Slider::new(&mut self.circuit_info.diode_cap, 1.0e-11..=1.0e-8).text("DC C"));
                    if  ((self.circuit_info.filter1_on || self.circuit_info.filter2_on)) && (vl.lost_focus() || vl.drag_released())||
                        ((self.circuit_info.filter1_on) && (dcr .lost_focus() || dcr.drag_released() || dcc .lost_focus() || dcc.drag_released())) ||
                        ((self.circuit_info.filter2_on) && (rcr .lost_focus() || rcr.drag_released() || rcc .lost_focus() || rcc.drag_released()))
                    {
                        self.send_circuit();
                    }

                    if ui.button("Reset").clicked(){    //Reset button to reset values to default
                        self.circuit_info = CircuitInfo::new(
                            2000.0,
                            3.0e-9,
                            470.0,
                            47.0e-9,
                            1.0,
                            self.circuit_info.filter1_on,
                            self.circuit_info.filter2_on
                        );
                        self.send_circuit();
                    }
                    if self.circuit_info.filter1_on && self.circuit_info.filter2_on{ //Image selector for circuit
                        ui.image(egui::include_image!("img/combined.png"));
                    }else if self.circuit_info.filter1_on{
                        ui.image(egui::include_image!("img/diode_clipper.png"));
                    }else if self.circuit_info.filter2_on{
                        ui.image(egui::include_image!("img/rc_lowpass.png"));
                    }else{
                        ui.image(egui::include_image!("img/in_out.png"));
                    }
                },
                None => ()
            }
            if self.sh_stopped{             //Check if stopped, outside body due to borrowing error
                self.stream_handler = None;
                self.sh_stopped = false;
            }
        });
    }
}