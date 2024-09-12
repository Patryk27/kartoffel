#![no_std]
#![no_main]

use kartoffel::*;

#[no_mangle]
fn main() {
    // NOTE If you get lost during the tutorial, see ../SPOILERS.md

    loop {
        motor_wait();
        motor_step();

        motor_wait();
        motor_step();

        motor_wait();
        motor_step();

        motor_wait();
        motor_turn_right();
    }
}
