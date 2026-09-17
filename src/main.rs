#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    //LED and button pins at https://docs.nordicsemi.com/viewer/attachment/Mg1yMHNCihJkguCGlcr6HQ/BnL90RfthQqTn3wWOLf3Sg-Mg1yMHNCihJkguCGlcr6HQ
    let mut led1 = Output::new(p.P0_28, Level::High, OutputDrive::Standard);
    let mut led2 = Output::new(p.P0_29, Level::High, OutputDrive::Standard);
    let mut led3 = Output::new(p.P0_30, Level::High, OutputDrive::Standard);
    let mut led4 = Output::new(p.P0_31, Level::High, OutputDrive::Standard);

    let mut cycle_num = 0;

    loop {
        info!("Cycle: {}", cycle_num);

        led3.set_high();
        led1.set_low();
        Timer::after_millis(250).await;

        led1.set_high();
        led2.set_low();
        Timer::after_millis(250).await;

        led2.set_high();
        led4.set_low();
        Timer::after_millis(250).await;

        led4.set_high();
        led3.set_low();
        Timer::after_millis(250).await;

        cycle_num += 1;
    }
}