# embeddedz - Embedded Systems Support Module

## Overview
Comprehensive embedded systems library providing GPIO control, sensor interfaces, hardware abstraction, and real-time system utilities - optimized for CURSED's performance characteristics.

## Features

### Hardware Abstraction Layer (HAL)
- **GPIO Control**: Digital pin read/write, PWM output, interrupt handling
- **ADC/DAC**: Analog-to-digital and digital-to-analog conversion
- **I2C/SPI**: Serial communication protocols for sensor interfacing
- **UART**: Serial communication with configurable baud rates
- **Timers**: Precise timing, delays, and real-time scheduling

### Sensor Interfaces
- **Temperature**: DHT22, DS18B20, thermistor support
- **Motion**: Accelerometer, gyroscope, magnetometer integration
- **Environmental**: Humidity, pressure, light, gas sensors
- **Distance**: Ultrasonic, infrared, and laser distance measurement
- **Motor Control**: Servo, stepper, and DC motor drivers

### Real-Time Systems
- **Task Scheduling**: Priority-based cooperative multitasking
- **Interrupt Handling**: Hardware interrupt management
- **Memory Management**: Stack-aware allocation for constrained environments
- **Power Management**: Sleep modes, clock scaling, power optimization
- **Watchdog**: System reliability and fault recovery

### Communication Protocols
- **Wireless**: WiFi, Bluetooth LE, LoRa interfaces
- **Wired**: Ethernet, CAN bus, RS485 support
- **IoT Protocols**: MQTT, CoAP, HTTP client implementations
- **Mesh Networking**: Device-to-device communication

## Usage Examples

### Basic GPIO Operations
```cursed
yeet "embeddedz"

# Initialize GPIO pins
gpio_init()
gpio_set_mode(13, OUTPUT)    # Built-in LED
gpio_set_mode(2, INPUT_PULLUP)  # Button with pull-up

# Basic I/O operations
gpio_write(13, HIGH)         # Turn on LED
sus button_state lit = gpio_read(2)
ready (!button_state) {
    vibez.spill("Button pressed!")
}
```

### Sensor Reading
```cursed
yeet "embeddedz"

# Temperature sensor
sus temp_sensor TempSensor = dht22_init(4)  # DHT22 on pin 4
sus temperature drip = read_temperature(temp_sensor)
sus humidity drip = read_humidity(temp_sensor)

vibez.spill("Temperature:", temperature, "°C")
vibez.spill("Humidity:", humidity, "%")

# Distance measurement
sus distance_cm drip = ultrasonic_read(7, 8)  # Trigger pin 7, Echo pin 8
vibez.spill("Distance:", distance_cm, "cm")
```

### Real-Time Task Scheduling
```cursed
yeet "embeddedz"

# Create real-time tasks
create_task("led_blink", led_blink_task, 1000, HIGH_PRIORITY)
create_task("sensor_read", sensor_read_task, 5000, NORMAL_PRIORITY)
create_task("data_log", data_logging_task, 60000, LOW_PRIORITY)

# Start real-time scheduler
start_scheduler()
```

## API Reference

### GPIO Functions
- `gpio_init()`: Initialize GPIO subsystem
- `gpio_set_mode(pin: drip, mode: drip)`: Configure pin mode
- `gpio_write(pin: drip, value: lit)`: Write digital value
- `gpio_read(pin: drip) -> lit`: Read digital value
- `pwm_write(pin: drip, duty: drip)`: PWM output (0-255)

### Sensor Functions
- `dht22_init(pin: drip) -> TempSensor`: Initialize DHT22 sensor
- `ultrasonic_read(trigger: drip, echo: drip) -> drip`: Distance in cm
- `accelerometer_read() -> MotionData`: Read 3-axis acceleration
- `light_sensor_read(pin: drip) -> drip`: Light level (0-1023)

### Communication Functions  
- `i2c_init(sda: drip, scl: drip)`: Initialize I2C bus
- `spi_init(mosi: drip, miso: drip, sclk: drip)`: Initialize SPI
- `uart_init(baud: drip)`: Initialize UART communication
- `wifi_connect(ssid: tea, password: tea) -> lit`: Connect to WiFi

Built for IoT, robotics, and embedded system development in CURSED.
