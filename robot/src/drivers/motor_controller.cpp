#include "drivers/pinout_definitions.h"
#include "drivers/motor_controller.h"
#include <iostream>

//extern "C" {
//  #include <pigpio.h>
//}

void drive_forward() {
	std::cout << "FORWARD\n";
	//gpioWrite(OUTPUT_PIN_E_RIGHT, 1);
	//gpioWrite(OUTPUT_PIN_A_RIGHT, 1);
	//gpioWrite(OUTPUT_PIN_B_RIGHT, 0);

	//gpioWrite(OUTPUT_PIN_E_LEFT, 1);
	//gpioWrite(OUTPUT_PIN_A_LEFT, 1);
	//gpioWrite(OUTPUT_PIN_B_LEFT, 0);
}

void drive_backward() {
	std::cout << "BACKWARD\n";
	//gpioWrite(OUTPUT_PIN_E_RIGHT, 1);
	//gpioWrite(OUTPUT_PIN_A_RIGHT, 0);
	//gpioWrite(OUTPUT_PIN_B_RIGHT, 1);

	//gpioWrite(OUTPUT_PIN_E_LEFT, 1);
	//gpioWrite(OUTPUT_PIN_A_LEFT, 0);
	//gpioWrite(OUTPUT_PIN_B_LEFT, 1);
}

void sharp_left() {
	std::cout << "LEFT\n";
	//gpioWrite(OUTPUT_PIN_E_RIGHT, 1);
	//gpioWrite(OUTPUT_PIN_A_RIGHT, 1);
	//gpioWrite(OUTPUT_PIN_B_RIGHT, 0);

	//gpioWrite(OUTPUT_PIN_E_LEFT, 1);
	//gpioWrite(OUTPUT_PIN_A_LEFT, 0);
	//gpioWrite(OUTPUT_PIN_B_LEFT, 1);
}


void sharp_right() {
	std::cout << "RIGHT\n";
	//gpioWrite(OUTPUT_PIN_E_RIGHT, 1);
	//gpioWrite(OUTPUT_PIN_A_RIGHT, 0);
	//gpioWrite(OUTPUT_PIN_B_RIGHT, 1);

	//gpioWrite(OUTPUT_PIN_E_LEFT, 1);;
	//gpioWrite(OUTPUT_PIN_A_LEFT, 1);
	//gpioWrite(OUTPUT_PIN_B_LEFT, 0);
}

void motors_off() {
	std::cout << "OFF\n";
	//gpioWrite(OUTPUT_PIN_E_RIGHT, 0);
	//gpioWrite(OUTPUT_PIN_A_RIGHT, 0);
	//gpioWrite(OUTPUT_PIN_B_RIGHT, 0);

	//gpioWrite(OUTPUT_PIN_E_LEFT, 0);
	//gpioWrite(OUTPUT_PIN_A_LEFT, 0);
	//gpioWrite(OUTPUT_PIN_B_LEFT, 0);
}


void initialize_motors() {
	//gpioSetMode(OUTPUT_PIN_E_RIGHT, PI_OUTPUT);
	//gpioSetMode(OUTPUT_PIN_B_RIGHT, PI_OUTPUT);
	//gpioSetMode(OUTPUT_PIN_A_RIGHT, PI_OUTPUT);
	//gpioSetMode(OUTPUT_PIN_E_LEFT, PI_OUTPUT);
	//gpioSetMode(OUTPUT_PIN_B_LEFT, PI_OUTPUT);
	//gpioSetMode(OUTPUT_PIN_A_LEFT, PI_OUTPUT);

	motors_off();
}
