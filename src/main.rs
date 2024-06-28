#![no_std]
#![no_main]

use kartoffel::*;

#[no_mangle]
fn main() {
    let mut random = timer_seed() as u64;

    loop {
        serial_send('.');

        // Wait for radar to become ready
        while !is_radar_ready() {
            //
        }

        // Scan a 3x3 area around the bot:
        //
        // ```
        // A B C
        // D @ F
        // G H I
        // ```
        radar_scan(3);

        // Wait for the scan to complete
        while !is_radar_ready() {
            //
        }

        // Read the scanned tiles - this will produce a 3x3 array like:
        //
        // [
        //   ['A', 'B', 'C'],
        //   ['D', '@', 'F'],
        //   ['G', 'H', 'I'],
        // ]
        //
        // (ofc. instead of 'A' there's going to be '.' if there's a floor, ' '
        // if there's no tile etc.)
        let radar = radar_read::<3>();

        // If there's a bot right in front of us, kill them!
        //
        // Otherwise just go to a random direction, if that direction is
        // walkable.
        if radar[0][1] == '@' {
            serial_send('!');

            // Wait for the arm to become ready
            while !is_arm_ready() {
                //
            }

            // Stab, stab, stab!
            arm_stab();
        } else {
            // Wait for the motor to become ready
            while !is_motor_ready() {
                //
            }

            // Move, move, move! (or rotate, rotate, rotate!)
            loop {
                random = rand(random);

                match random % 3 {
                    // Go forward (if the tile in front of us is passable)
                    0 if radar[0][1] != ' ' => {
                        serial_send('^');
                        motor_step();
                        break;
                    }

                    // Turn left (if it makes sense to later move forward there)
                    1 if radar[1][0] != ' ' => {
                        serial_send('<');
                        motor_turn(-1);
                        break;
                    }

                    // Turn right (ditto)
                    2 if radar[1][2] != ' ' => {
                        serial_send('>');
                        motor_turn(1);
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
