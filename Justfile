set dotenv-load

acceptance-tests:
    cargo clippy --no-deps --all-features -- -Dwarnings
    cargo build --all-features

flash:
    cargo build --release
    ravedude --port $ARDUINO_PORT $ARDUINO_BOARD target/avr-atmega328p/release/water-sensor-alert.elf