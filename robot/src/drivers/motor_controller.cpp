#include "drivers/pinout_definitions.h"
#include "drivers/motor_controller.h"
#include <iostream>

extern "C" {
  #include <pigpio.h>
}

#define PI_OUTPUT 1

void drive_forward() {
	std::cout << "FORWARD\n";
	gpioWrite(OUTPUT_E_RIGHT, 1);
	gpioWrite(OUTPUT_IN_3_RIGHT, 1);
	gpioWrite(OUTPUT_IN_4_RIGHT, 0);

	gpioWrite(OUTPUT_E_LEFT, 1);
	gpioWrite(OUTPUT_IN_1_LEFT, 1);
	gpioWrite(OUTPUT_IN_2_LEFT, 0);
}

void drive_backward() {
	std::cout << "BACKWARD\n";
	gpioWrite(OUTPUT_E_RIGHT, 1);
	gpioWrite(OUTPUT_IN_3_RIGHT, 0);
	gpioWrite(OUTPUT_IN_4_RIGHT, 1);

	gpioWrite(OUTPUT_E_LEFT, 1);
	gpioWrite(OUTPUT_IN_1_LEFT, 0);
	gpioWrite(OUTPUT_IN_2_LEFT, 1);
}

void sharp_left() {
	std::cout << "LEFT\n";
	gpioWrite(OUTPUT_E_RIGHT, 1);
	gpioWrite(OUTPUT_IN_3_RIGHT, 1);
	gpioWrite(OUTPUT_IN_4_RIGHT, 0);

	gpioWrite(OUTPUT_E_LEFT, 1);
	gpioWrite(OUTPUT_IN_1_LEFT, 0);
	gpioWrite(OUTPUT_IN_2_LEFT, 1);
}


void sharp_right() {
	std::cout << "RIGHT\n";
	gpioWrite(OUTPUT_E_RIGHT, 1);
	gpioWrite(OUTPUT_IN_3_RIGHT, 0);
	gpioWrite(OUTPUT_IN_4_RIGHT, 1);

	gpioWrite(OUTPUT_E_LEFT, 1);;
	gpioWrite(OUTPUT_IN_1_LEFT, 1);
	gpioWrite(OUTPUT_IN_2_LEFT, 0);
}

void motors_off() {
	std::cout << "OFF\n";
	gpioWrite(OUTPUT_E_RIGHT, 0);
	gpioWrite(OUTPUT_IN_3_RIGHT, 0);
	gpioWrite(OUTPUT_IN_4_RIGHT, 0);

	gpioWrite(OUTPUT_E_LEFT, 0);
	gpioWrite(OUTPUT_IN_1_LEFT, 0);
	gpioWrite(OUTPUT_IN_2_LEFT, 0);
}


void initialize_motors() {
	gpioInitialise();
	gpioSetMode(OUTPUT_E_RIGHT, PI_OUTPUT);
	gpioSetMode(OUTPUT_IN_4_RIGHT, PI_OUTPUT);
	gpioSetMode(OUTPUT_IN_3_RIGHT, PI_OUTPUT);
	gpioSetMode(OUTPUT_E_LEFT, PI_OUTPUT);
	gpioSetMode(OUTPUT_IN_2_LEFT, PI_OUTPUT);
	gpioSetMode(OUTPUT_IN_1_LEFT, PI_OUTPUT);
	motors_off();
}
