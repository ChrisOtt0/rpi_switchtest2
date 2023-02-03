use rppal::gpio::{Gpio, InputPin, OutputPin};

fn main() {
    let gpio: Gpio = Gpio::new()
        .unwrap_or_else(|e| panic!("Failed to create struct Gpio: {e}"));

    let input: InputPin = gpio.get(18)
        .unwrap_or_else(|e| panic!("Failed to create struct Pin: {e}"))
        .into_input_pulldown();

    let mut output: OutputPin = gpio.get(26)
        .unwrap_or_else(|e| panic!("Failed to create struct Pin: {e}"))
        .into_output();

    loop {
        let state = input.is_low();

        if state {
            output.set_high();
        } else {
            output.set_low();
        }
    };
}
