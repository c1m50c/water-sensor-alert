#define WATER_SENSOR_ALERTING_THRESHOLD 100
#define WATER_SENSOR_SAMPLING_RATE 250
#define WATER_SENSOR_PIN A5

int water_sensor_rx = 0;

void setup() {
    Serial.begin(9600);
}

void loop() {
    water_sensor_rx = analogRead(WATER_SENSOR_PIN);
    Serial.println(water_sensor_rx);

    if (water_sensor_rx >= WATER_SENSOR_ALERTING_THRESHOLD) {
        // TODO: Alerting logic goes here...
    }

    delay(WATER_SENSOR_SAMPLING_RATE);
}