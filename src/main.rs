mod gpuinfo;
mod cpuinfo;
use crate::gpuinfo::*;
use crate::cpuinfo::*;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::Nvml;

fn main() {
    let gpu: Result<Nvml, NvmlError> = Nvml::init();
    if gpu.is_ok()
    {
        let stats: GPUInfo = get_info(&gpu.unwrap().device_by_index(0).unwrap());
        println!("{} {}", stats.name.unwrap(), stats.temp.unwrap());
    }

    read_cpu();
}
