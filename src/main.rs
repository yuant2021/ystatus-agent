use systemstat::{Platform, System};
fn fetch_cpu() {
    let sys = System::new();
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            std::thread::sleep(std::time::Duration::from_secs(1));
            match cpu.done() {
                Ok(cpu) => {
                    println!("CPU Load: {:?}", cpu);
                }
                Err(x) => {
                    eprintln!("Error: {:?}", x);
                }
            }
        }
        Err(_) => todo!(),
    }
}
fn main() {
    println!("Hello, world!");
    fetch_cpu();
}
