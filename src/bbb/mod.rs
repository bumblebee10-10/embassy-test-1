use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pin, Pull};
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low);

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(350)).await;
        led.set_low();
        Timer::after(Duration::from_millis(350)).await;
    }
}
