use systemstat::{System, Platform, saturating_sub_bytes};

pub struct CPUInfo {
    pub name: String,
    pub temp: f32,
}

pub fn read_cpu() -> () {
    let sys = System::new();
    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x)
    }

}