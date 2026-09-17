#![no_std]
#![no_main]

use core::cell::Cell;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive, Pull};
use embassy_nrf::gpiote::{InputChannel, InputChannelPolarity};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    //LED and button pins at https://docs.nordicsemi.com/viewer/attachment/Mg1yMHNCihJkguCGlcr6HQ/BnL90RfthQqTn3wWOLf3Sg-Mg1yMHNCihJkguCGlcr6HQ
    let mut leds = [
        Output::new(p.P0_28, Level::High, OutputDrive::Standard),
        Output::new(p.P0_29, Level::High, OutputDrive::Standard),
        Output::new(p.P0_31, Level::High, OutputDrive::Standard),
        Output::new(p.P0_30, Level::High, OutputDrive::Standard),
    ];

    let mut ch1 = InputChannel::new(p.GPIOTE_CH0, p.P0_23, Pull::Up, InputChannelPolarity::HiToLo);

    let mut cycle_num = 0;
    let mut led_i: i32 = 0;
    let mut to_next_led: Cell<i32> = Cell::new(1);

    let button1 = async {
        loop {
            ch1.wait().await;
            info!("Button 1 pressed");
            to_next_led.set(to_next_led.get() * -1);
        }
    };

    let leds_loop = async {
        loop {
            info!("Cycle: {}", cycle_num);

            leds[led_i as usize].set_high();
            led_i += to_next_led.get();

            led_i = match led_i {
                4 => 0,
                -1 => 3,
                n => n
            };

            leds[led_i as usize].set_low();
            Timer::after_millis(250).await;

            cycle_num += 1;
        }
    };

    embassy_futures::join::join(button1, leds_loop).await;
}