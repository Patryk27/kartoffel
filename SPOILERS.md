# Spoilers

This file contains solutions for the tutorial.

## Commented-out

```rust
#[no_mangle]
fn main() {
    loop {
        motor_wait();
        motor_step();

        motor_wait();
        motor_step();

        motor_wait();
        motor_step();

        motor_wait();
        // motor_turn_right();
    }
}
```

## Line-follower

```rust
#[no_mangle]
fn main() {
    loop {
        radar_wait();

        let scan = radar_scan_3x3();

        if scan.at(0, -1) == '.' {
            motor_wait();
            motor_step();
        } else if scan.at(-1, 0) == '.' {
            motor_wait();
            motor_turn_left();
        } else if scan.at(1, 0) == '.' {
            motor_wait();
            motor_turn_right();
        }
    }
}
```

## Enemy-stabber

```rust
#[no_mangle]
fn main() {
    loop {
        radar_wait();

        let scan = radar_scan_3x3();

        if scan.at(0, -1) == '@' {
            arm_wait();
            arm_stab();
        } else if scan.at(-1, 0) == '@' {
            motor_wait();
            motor_turn_left();
        } else if scan.at(1, 0) == '@' {
            motor_wait();
            motor_turn_right();
        } else {
            motor_wait();
            motor_step();
        }
    }
}
```
