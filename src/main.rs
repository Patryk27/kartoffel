#![no_std]
#![no_main]

use kartoffel::*;

#[no_mangle]
fn main() {
    loop {
        radar_wait();

        let scan = radar_scan_5x5();

        // If someone's directly in front of us, stab, stab, stab!
        if scan[1][2] == '@' {
            arm_wait();
            arm_stab();
            motor_wait();
            motor_step();
            continue;
        }

        // If someone's to our left, turn left
        if scan[2][1] == '@' || scan[2][0] == '@' {
            motor_turn_left();
            motor_wait();
            continue;
        }

        // If someone's to our right, turn right
        if scan[2][3] == '@' || scan[2][4] == '@' {
            motor_turn_right();
            motor_wait();
            continue;
        }

        // If someone's behind us, turn left (or right, would be the same)
        if scan[3][2] == '@' || scan[4][2] == '@' {
            motor_turn_left();
            motor_wait();
            continue;
        }

        // Otherwise continue moving in our current direction
        motor_wait();
        motor_step();
    }
}
