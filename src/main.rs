mod cpu;
mod memory;
mod storage;
mod network;

fn main() {
    println!("This stability test is split up into multiple different parts: CPU (single-thread), RAM, Storage & Network.\n");
    // CPU
    cpu::run();
    // RAM
    memory::run();
    // Storage
    storage::run();
    // Network
    network::run();
}
