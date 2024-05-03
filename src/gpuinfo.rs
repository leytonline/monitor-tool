use nvml_wrapper::enum_wrappers::device::{
    TemperatureSensor,
};
use nvml_wrapper::error::NvmlError as NvmlError;

pub struct GPUInfo {
    pub name: Result<String, NvmlError>,
    pub temp: Result<u32, NvmlError>,
}

pub fn get_info(dev: &nvml_wrapper::Device) -> GPUInfo {
    let info: GPUInfo = GPUInfo {
        name: dev.name(),
        temp: dev.temperature(TemperatureSensor::Gpu),
    };
    return info;
}