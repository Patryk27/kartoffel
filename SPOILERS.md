# Spoilers

This file contains solutions for the tutorial.

## Commenting

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

## Radaring

```rust
#[no_mangle]
fn main() {
    loop {
        radar_wait();

        let scan = radar_scan_3x3();

        if scan[0][1] == '.' {
            motor_wait();
            motor_step();
        } else if scan[1][0] == '.' {
            motor_wait();
            motor_turn_left();
        } else if scan[1][2] == '.' {
            motor_wait();
            motor_turn_right();
        }
    }
}
```

## Stabbing

```rust
#[no_mangle]
fn main() {
    loop {
        radar_wait();

        let scan = radar_scan_3x3();

        if scan[0][1] == '@' {
            arm_wait();
            arm_stab();
        } else if scan[1][0] == '@' {
            motor_wait();
            motor_turn_left();
        } else if scan[1][2] == '@' {
            motor_wait();
            motor_turn_right();
        } else {
            motor_wait();
            motor_step();
        }
    }
}
```
