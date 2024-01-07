set dotenv-path := ".env"
set dotenv-load := true

# Compiles and uploads the `water-sensor-alert` sketch to the given `$ARDUINO_BOARD` and `$ARDUINO_PORT`
upload:
    arduino-cli compile -u -b $ARDUINO_BOARD -p $ARDUINO_PORT ./water-sensor-alert