#![no_std]
#![no_main]

use kartoffel::*;

#[no_mangle]
fn main() {
    let mut random = timer_seed() as u64;

    loop {
        serial_send('.');

        // Wait for radar to become ready
        radar_wait();

        // Scan a 3x3 area around the bot - this will produce a 3x3 array like:
        //
        // [
        //   [' ', ' ', ' '],
        //   [' ', '@', ' '],
        //   [' ', ' ', ' '],
        // ]
        let scan = radar_scan_3x3();

        // If there's a bot right in front of us, kill them!
        //
        // Otherwise just go to a random direction, if that direction is
        // walkable.
        if scan[0][1] == '@' {
            serial_send('!');
            arm_wait();
            arm_stab();
        } else {
            motor_wait();

            loop {
                random = rand(random);

                match random % 10 {
                    // 80% chance to go forward (if the tile in front of us is
                    // passable)
                    0..=7 if scan[0][1] != ' ' => {
                        serial_send('^');
                        motor_step();
                        break;
                    }

                    // 10% chance to turn left (if it makes sense to later move
                    // forward there)
                    8 if scan[1][0] != ' ' => {
                        serial_send('<');
                        motor_turn_left();
                        break;
                    }

                    // 10% chance to turn right (ditto)
                    9 if scan[1][2] != ' ' => {
                        serial_send('>');
                        motor_turn_right();
                        break;
                    }

                    _ => (),
                }
            }
        }
    }
}

/// Gets the next pseudo-random number using a linear congruential generator.
fn rand(mut r: u64) -> u64 {
    r *= 1664525;
    r += 1013904223;
    r
}
