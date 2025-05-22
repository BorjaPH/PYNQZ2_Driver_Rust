mod driver;

use driver::Gpio;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Inicializando LEDs PYNQ-Z2...");

    let gpio = Gpio::new().expect("Error inicializando GPIO");

    for i in 0..4 {
        let val = 1 << i;
        println!("Encendiendo LED {}", i);
        gpio.write(val);
        thread::sleep(Duration::from_millis(1000));
    }

    gpio.write(0);
    println!("Todos los LEDs apagados.");
}
