# embeddedz/examples.csd - Embedded Systems Examples
# Comprehensive examples for IoT and embedded development

yeet "embeddedz/core"
yeet "vibez"

# Example 1: Basic GPIO operations
slay demo_gpio_basic() {
    vibez.spill("\n=== Basic GPIO Operations ===")
    
    # Configure pins
    gpio_set_mode(13, OUTPUT)    # Built-in LED
    gpio_set_mode(2, INPUT_PULLUP)  # Button with pullup
    gpio_set_mode(3, INPUT)      # Simple input
    
    # Basic output operations
    vibez.spill("Turning LED on...")
    gpio_write(13, HIGH)
    delay_milliseconds(1000)
    
    vibez.spill("Turning LED off...")
    gpio_write(13, LOW)
    delay_milliseconds(1000)
    
    # Read inputs
    sus button_state lit = gpio_read(2)
    sus input_state lit = gpio_read(3)
    
    vibez.spill("Button state (inverted due to pullup):", !button_state)
    vibez.spill("Input state:", input_state)
}

# Example 2: PWM motor control
slay demo_pwm_motor_control() {
    vibez.spill("\n=== PWM Motor Control ===")
    
    sus motor_pin drip = 9
    gpio_set_mode(motor_pin, OUTPUT)
    
    vibez.spill("Motor speed control demonstration:")
    
    # Gradual speed increase
    bestie (sus speed drip = 0; speed <= 255; speed = speed + 51) {
        vibez.spill("Setting motor speed:", speed, "/255")
        pwm_write(motor_pin, speed)
        delay_milliseconds(500)
    }
    
    # Stop motor
    pwm_write(motor_pin, 0)
    vibez.spill("Motor stopped")
}

# Example 3: Servo control
slay demo_servo_control() {
    vibez.spill("\n=== Servo Motor Control ===")
    
    sus servo_pin drip = 6
    gpio_set_mode(servo_pin, OUTPUT)
    
    vibez.spill("Servo angle control:")
    
    # Sweep servo from 0 to 180 degrees
    bestie (sus angle drip = 0; angle <= 180; angle = angle + 30) {
        vibez.spill("Setting servo angle:", angle, "degrees")
        servo_write(servo_pin, angle)
        delay_milliseconds(1000)
    }
    
    # Return to center
    servo_write(servo_pin, 90)
    vibez.spill("Servo returned to center (90 degrees)")
}

# Example 4: Stepper motor control
slay demo_stepper_control() {
    vibez.spill("\n=== Stepper Motor Control ===")
    
    sus step_pin drip = 8
    sus dir_pin drip = 7
    
    # Forward rotation
    vibez.spill("Stepper motor - 100 steps forward")
    stepper_step(step_pin, dir_pin, 100, 10)
    
    delay_milliseconds(1000)
    
    # Reverse rotation
    vibez.spill("Stepper motor - 100 steps reverse")
    stepper_step(step_pin, dir_pin, -100, 10)
    
    vibez.spill("Stepper demo complete")
}

# Example 5: Temperature and humidity monitoring
slay demo_dht22_sensor() {
    vibez.spill("\n=== DHT22 Temperature/Humidity Sensor ===")
    
    sus sensor TempSensor = dht22_init(4)
    
    # Take multiple readings
    bestie (sus i drip = 0; i < 5; i = i + 1) {
        vibez.spill("Reading", i + 1, ":")
        
        sus temperature drip = read_temperature(sensor)
        sus humidity drip = read_humidity(sensor)
        
        vibez.spill("  Temperature:", temperature, "°C")
        vibez.spill("  Humidity:", humidity, "%")
        
        # Calculate heat index (simplified)
        sus heat_index drip = temperature + (humidity / 10)
        vibez.spill("  Heat Index:", heat_index, "°C")
        
        delay_milliseconds(3000)  # DHT22 requires 2+ second intervals
    }
}

# Example 6: Ultrasonic distance measurement
slay demo_ultrasonic_sensor() {
    vibez.spill("\n=== Ultrasonic Distance Sensor ===")
    
    sus trigger_pin drip = 7
    sus echo_pin drip = 8
    
    bestie (sus i drip = 0; i < 10; i = i + 1) {
        sus distance drip = ultrasonic_read(trigger_pin, echo_pin)
        
        vibez.spill("Distance measurement", i + 1, ":", distance, "cm")
        
        # Categorize distance
        ready (distance < 10) {
            vibez.spill("  Status: Very close!")
        } otherwise ready (distance < 50) {
            vibez.spill("  Status: Near")
        } otherwise ready (distance < 200) {
            vibez.spill("  Status: Far")
        } otherwise {
            vibez.spill("  Status: Very far or no object")
        }
        
        delay_milliseconds(500)
    }
}

# Example 7: Motion sensor (accelerometer/gyroscope)
slay demo_motion_sensor() {
    vibez.spill("\n=== Motion Sensor (IMU) ===")
    
    bestie (sus i drip = 0; i < 5; i = i + 1) {
        sus motion MotionData = accelerometer_read()
        
        vibez.spill("Motion reading", i + 1, ":")
        vibez.spill("  Acceleration - X:", motion.accel_x, "Y:", motion.accel_y, "Z:", motion.accel_z)
        vibez.spill("  Gyroscope - X:", motion.gyro_x, "Y:", motion.gyro_y, "Z:", motion.gyro_z)
        vibez.spill("  Magnetometer - X:", motion.mag_x, "Y:", motion.mag_y, "Z:", motion.mag_z)
        
        # Calculate total acceleration
        sus total_accel drip = sqrt_approx(
            (motion.accel_x * motion.accel_x) + 
            (motion.accel_y * motion.accel_y) + 
            (motion.accel_z * motion.accel_z)
        )
        vibez.spill("  Total acceleration:", total_accel)
        
        delay_milliseconds(1000)
    }
}

# Example 8: Light sensor monitoring
slay demo_light_sensor() {
    vibez.spill("\n=== Light Sensor Monitoring ===")
    
    sus light_pin drip = 0  # Analog pin A0
    
    bestie (sus i drip = 0; i < 8; i = i + 1) {
        sus light_level drip = light_sensor_read(light_pin)
        sus percentage drip = (light_level * 100) / 1023
        
        vibez.spill("Light level:", light_level, "/1023 (", percentage, "%)")
        
        # Categorize light level
        ready (light_level < 200) {
            vibez.spill("  Condition: Dark")
        } otherwise ready (light_level < 500) {
            vibez.spill("  Condition: Dim")
        } otherwise ready (light_level < 800) {
            vibez.spill("  Condition: Bright")
        } otherwise {
            vibez.spill("  Condition: Very bright")
        }
        
        delay_milliseconds(500)
    }
}

# Example 9: I2C communication
slay demo_i2c_communication() {
    vibez.spill("\n=== I2C Communication ===")
    
    sus i2c_device I2cDevice = i2c_init(20, 21)  # SDA=20, SCL=21
    sus device_address drip = 0x48  # Example temperature sensor address
    
    # Write configuration to device
    sus config_data drip[value] = [0x01, 0x60, 0xA0]  # Example config
    sus write_success lit = i2c_write(i2c_device, device_address, config_data)
    
    ready (write_success) {
        vibez.spill("I2C configuration written successfully")
        
        # Read data from device
        sus read_data drip[value] = i2c_read(i2c_device, device_address, 4)
        
        vibez.spill("I2C data read:")
        bestie (sus i drip = 0; i < len_drip_array(read_data); i = i + 1) {
            vibez.spill("  Byte", i, ":", read_data[i])
        }
    } otherwise {
        vibez.spill("I2C communication failed")
    }
}

# Example 10: SPI communication
slay demo_spi_communication() {
    vibez.spill("\n=== SPI Communication ===")
    
    sus spi_device SpiDevice = spi_init(11, 12, 13, 10)  # MOSI, MISO, SCLK, CS
    
    # Send command to SPI device
    sus command_data drip[value] = [0x9F, 0x00, 0x00, 0x00]  # Read ID command
    sus response drip[value] = spi_transfer(spi_device, command_data)
    
    vibez.spill("SPI command sent:", command_data[0])
    vibez.spill("SPI response received:")
    bestie (sus i drip = 0; i < len_drip_array(response); i = i + 1) {
        vibez.spill("  Byte", i, ":", response[i])
    }
}

# Example 11: UART communication
slay demo_uart_communication() {
    vibez.spill("\n=== UART Communication ===")
    
    sus uart UartDevice = uart_init(9600)  # 9600 baud
    
    # Send data
    uart_write(uart, "Hello from CURSED embedded system!")
    uart_write(uart, "Sensor data: temp=25.3C, humidity=60%")
    
    # Read data (simulated)
    sus received tea = uart_read(uart)
    vibez.spill("UART received:", received)
}

# Example 12: WiFi connectivity
slay demo_wifi_connectivity() {
    vibez.spill("\n=== WiFi Connectivity ===")
    
    # Connect to WiFi
    sus connected lit = wifi_connect("MyWiFiNetwork", "password123")
    
    ready (connected) {
        sus status WifiConfig = wifi_status()
        vibez.spill("WiFi connected successfully!")
        vibez.spill("  SSID:", status.ssid)
        vibez.spill("  IP Address:", status.ip_address)
        vibez.spill("  Signal Strength:", status.signal_strength, "dBm")
        
        # Simulate sending data to cloud
        vibez.spill("Sending sensor data to cloud...")
        delay_milliseconds(2000)
        vibez.spill("Data uploaded successfully!")
        
        # Disconnect
        wifi_disconnect()
    } otherwise {
        vibez.spill("WiFi connection failed")
    }
}

# Example 13: Real-time task scheduling
slay demo_real_time_tasks() {
    vibez.spill("\n=== Real-Time Task Scheduling ===")
    
    # Create multiple tasks with different priorities and intervals
    create_task("led_blink", "led_blink_task", 1000, HIGH_PRIORITY)
    create_task("sensor_read", "sensor_read_task", 5000, NORMAL_PRIORITY)
    create_task("data_log", "data_logging_task", 10000, LOW_PRIORITY)
    
    vibez.spill("Real-time tasks created. Starting scheduler...")
    vibez.spill("Note: In real implementation, scheduler would run indefinitely")
    
    # Simulate scheduler for a limited time
    bestie (sus i drip = 0; i < 5; i = i + 1) {
        vibez.spill("\nScheduler tick", i + 1)
        
        # Manually execute tasks for demonstration
        led_blink_task()
        
        ready (i % 2 == 0) {
            sensor_read_task()
        }
        
        ready (i % 3 == 0) {
            data_logging_task()
        }
        
        delay_milliseconds(2000)
    }
    
    vibez.spill("Task scheduling demonstration complete")
}

# Example 14: Power management
slay demo_power_management() {
    vibez.spill("\n=== Power Management ===")
    
    # Normal operation
    set_power_mode(1)  # Normal mode
    vibez.spill("System running in normal power mode")
    
    # Take some sensor readings
    sus temp_sensor TempSensor = dht22_init(4)
    sus temperature drip = read_temperature(temp_sensor)
    vibez.spill("Temperature reading:", temperature, "°C")
    
    # Enter power saving mode
    set_power_mode(2)  # Power save mode
    vibez.spill("Switching to power save mode")
    
    # Reduce activity
    delay_milliseconds(1000)
    
    # Deep sleep demonstration
    vibez.spill("Entering deep sleep for 3 seconds...")
    deep_sleep(3000)
    
    # Wake up and resume
    set_power_mode(1)  # Back to normal
    vibez.spill("System awakened and resumed normal operation")
}

# Example 15: IoT sensor node simulation
slay demo_iot_sensor_node() {
    vibez.spill("\n=== IoT Sensor Node Simulation ===")
    
    # Initialize all sensors
    sus temp_sensor TempSensor = dht22_init(4)
    sus light_pin drip = 0
    
    # Connect to WiFi
    sus connected lit = wifi_connect("IoT_Network", "sensor123")
    
    ready (connected) {
        vibez.spill("IoT sensor node online!")
        
        # Main sensor loop
        bestie (sus cycle drip = 0; cycle < 3; cycle = cycle + 1) {
            vibez.spill("\n--- Sensor Cycle", cycle + 1, "---")
            
            # Read all sensors
            sus temperature drip = read_temperature(temp_sensor)
            sus humidity drip = read_humidity(temp_sensor)
            sus light_level drip = light_sensor_read(light_pin)
            sus motion MotionData = accelerometer_read()
            
            # Create sensor data packet
            vibez.spill("Sensor readings:")
            vibez.spill("  Temperature:", temperature, "°C")
            vibez.spill("  Humidity:", humidity, "%")
            vibez.spill("  Light Level:", (light_level * 100) / 1023, "%")
            vibez.spill("  Motion detected:", motion.accel_x > 50 || motion.accel_y > 50)
            
            # Simulate data transmission
            vibez.spill("Transmitting data to IoT platform...")
            delay_milliseconds(1000)
            vibez.spill("Data transmission successful")
            
            # Check for alerts
            ready (temperature > 30) {
                vibez.spill("ALERT: High temperature detected!")
            }
            
            ready (light_level < 100) {
                vibez.spill("ALERT: Low light conditions")
            }
            
            delay_milliseconds(5000)  # Wait before next cycle
        }
        
        wifi_disconnect()
    } otherwise {
        vibez.spill("Failed to connect to IoT network")
    }
}

# Utility function for square root approximation
slay sqrt_approx(value drip) drip {
    # Simple square root approximation using Newton's method
    ready (value <= 0) {
        damn 0
    }
    
    sus x drip = value / 2
    bestie (sus i drip = 0; i < 10; i = i + 1) {
        x = (x + (value / x)) / 2
    }
    damn x
}

# Main demonstration runner
slay run_all_embedded_demos() {
    vibez.spill("CURSED Embedded Systems Library Demonstration")
    vibez.spill("=============================================")
    vibez.spill("Pure CURSED implementation for IoT and embedded development")
    
    demo_gpio_basic()
    demo_pwm_motor_control()
    demo_servo_control()
    demo_stepper_control()
    demo_dht22_sensor()
    demo_ultrasonic_sensor()
    demo_motion_sensor()
    demo_light_sensor()
    demo_i2c_communication()
    demo_spi_communication()
    demo_uart_communication()
    demo_wifi_connectivity()
    demo_real_time_tasks()
    demo_power_management()
    demo_iot_sensor_node()
    
    vibez.spill("\n=== All Embedded Demos Complete ===")
    vibez.spill("Embedded systems functionality demonstrated successfully!")
    vibez.spill("Module info:", get_module_info())
}

# Run demos when module is executed
run_all_embedded_demos()
