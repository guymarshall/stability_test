mod cpu;
mod memory;
mod storage;
mod network;

fn main() {
    println!("This stability test is split up into multiple different parts: CPU (single-thread), RAM, Storage & Network.\n");
    cpu::run();
    // memory::run();
    storage::run();
    network::run();
}
