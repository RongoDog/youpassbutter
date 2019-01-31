#ifndef MOTOR_CONTROLLER_H
#define MOTOR_CONTROLLER_H

extern void drive_forward(unsigned int duty_cycle);
extern void drive_backward(unsigned int duty_cycle);
extern void sharp_left(unsigned int duty_cycle);
extern void sharp_right(unsigned int duty_cycle);
extern void motors_off();
extern void initialize_motors();

#endif // MOTOR_CONTROLLER_H
