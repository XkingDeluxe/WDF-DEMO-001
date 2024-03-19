//Dependencies/crates
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SizedSample, Stream};
use std::fs::File;
use crate::circuithandler::*;

pub struct FileHandler{     //File handler body
    audio_loop: Vec<f32>,   //Selected looped audio file
    current_sample_index:f32,   //Sample index
    circuit: CircuitHandler,    //Circuit handler
    filter1_on: bool,   //|
    filter2_on: bool    //|Filters on or off
}
impl FileHandler {
    pub fn new(path: &str, _circuit:CircuitHandler) -> FileHandler{ //Intitialize Filehandler
        let mut f = match File::open(path) {    //Try to open specified file
            Ok(f) => f,
            Err(_e) => panic!("Something went wrong while opening file: {}", path)
        };
        let (_header, bitdepth) = match wav::read(&mut f){  //Read file contents
            Ok((h, bd)) => (h, bd),
            Err(_e) => panic!("Something went wrong while opening file: {}", path)
        };
        FileHandler{
            audio_loop: match bitdepth.try_into_thirty_two_float() {    //Push data into vector
                Ok(v) => v,
                Err(_e)=> panic!("Error while converting bitmap into vector:")
            },
            current_sample_index: 0.0,
            circuit:_circuit,
            filter1_on: false,
            filter2_on: false
        }
    }
    fn advance_sample(&mut self){   //Set sampleindex +1. Set to 0 when index == length-1
        self.current_sample_index = (self.current_sample_index + 1.0) % self.audio_loop.len() as f32;
    }
    fn tick(&mut self) -> f32{  //Tick to calculate input to output
        let samp = self.audio_loop[self.current_sample_index as usize];
        self.advance_sample();
        self.circuit.tick(samp, self.filter1_on, self.filter2_on)
        
    }
    fn update_circuit(&mut self, _new_circuit:CircuitInfo){ //Update any value changes
        self.filter1_on = _new_circuit.filter1_on;
        self.filter2_on = _new_circuit.filter2_on;
        self.circuit.update(_new_circuit);
    }
}

pub struct StreamHandler{
    pub stream:Stream
}
impl StreamHandler {
    pub fn new(_sender: &mut Option<crossbeam_channel::Sender<CircuitInfo>>, fh: FileHandler) -> StreamHandler{ //New streamhandler
        let (_host, device, config) = Self::host_device_setup().unwrap();
        StreamHandler{
            stream:match config.sample_format() {   //Check stream format of output device
                cpal::SampleFormat::I8 => Self::make_stream::<i8>(_sender, fh,&device, &config.into()),
                cpal::SampleFormat::I16 => Self::make_stream::<i16>(_sender,fh,&device, &config.into()),
                cpal::SampleFormat::I32 => Self::make_stream::<i32>(_sender,fh,&device, &config.into()),
                cpal::SampleFormat::I64 => Self::make_stream::<i64>(_sender,fh,&device, &config.into()),
                cpal::SampleFormat::U8 => Self::make_stream::<u8>(_sender,fh,&device, &config.into()),
                cpal::SampleFormat::U16 => Self::make_stream::<u16>(_sender,fh,&device, &config.into()),
                cpal::SampleFormat::U32 => Self::make_stream::<u32>(_sender,fh,&device, &config.into()),
                cpal::SampleFormat::U64 => Self::make_stream::<u64>(_sender,fh,&device, &config.into()),
                cpal::SampleFormat::F32 => Self::make_stream::<f32>(_sender,fh,&device, &config.into()),
                cpal::SampleFormat::F64 => Self::make_stream::<f64>(_sender,fh,&device, &config.into()),
                sample_format => Err(anyhow::Error::msg(format!(
                    "Unsupported sample format '{sample_format}'"
                ))),
            }.unwrap()
        }
    }
    pub fn play(&mut self){ //Stream to output device
        match self.stream.play(){
            Ok(()) => (),
            Err(e) => panic!("Error playing streamhandler: {}", e)
        }
    }
    pub fn pause(&mut self){    //Pause stream to output device
        match self.stream.pause(){
            Ok(()) => (),
            Err(e) => panic!("Error pausing streamhandler: {}", e)
        }
    }
    pub fn stop(&mut self){     //Stop stream to output device
        match self.stream.pause(){
            Ok(()) => (),
            Err(e) => panic!("Error pausing streamhandler: {}", e)
        }
    }


    pub fn host_device_setup(   //Setup for device output
    ) -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
        let host = cpal::default_host();    //Check default audio output adaptor
    
        let device = host
            .default_output_device()
            .ok_or_else(||anyhow::Error::msg("Default output device is not available"))?;
        println!("Output device : {}", device.name()?);
    
        let config = device.default_output_config()?;
        println!("Default output config : {:?}", config);
        Ok((host, device, config))
    }     
    fn make_stream<T>(_sender: &mut Option<crossbeam_channel::Sender<CircuitInfo>>, mut fh: FileHandler, device: &cpal::Device, config: &cpal::StreamConfig) -> Result<Stream, anyhow::Error>
    where
        T: SizedSample + FromSample<f32>,   //Stream data
    {
        //let sample_rate = config.sample_rate.0;
        let channels = config.channels as usize;
        let err_fn = |err| eprintln!("Stream error: {}", err);

        let (send, recv) = crossbeam_channel::unbounded::<CircuitInfo>();   //Async communication
        *_sender = Some(send);
        let stream = device.build_output_stream(
            config,
            move |output: &mut [T], _: &cpal::OutputCallbackInfo|{                  //This is an external thread and because the async communication, it is possible to exchange data (changes)
                if !recv.is_empty(){
                    let circuit_info = recv.try_recv().expect("Failed to read");
                    fh.update_circuit(circuit_info);                                    //Update circuit
                }
                Self::write_frame(output, &mut fh, channels)
            }, err_fn, None)?;
        Ok(stream)
    }

    fn write_frame<T>(output: &mut [T], audio_loop: &mut FileHandler, channels: usize)
    where
        T: Sample + FromSample<f32>,
    {
        for frame in output.chunks_mut(channels) {  //Write samples to output device
            let value = T::from_sample(audio_loop.tick());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
}