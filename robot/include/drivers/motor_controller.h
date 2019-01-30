#ifndef MOTOR_CONTROLLER_H
#define MOTOR_CONTROLLER_H

extern void drive_forward(unsigned int leftCycle, unsigned int rightCycle);
extern void drive_backward(unsigned int leftCycle, unsigned int rightCycle);
extern void sharp_left(unsigned int leftCycle, unsigned int rightCycle);
extern void sharp_right(unsigned int leftCycle, unsigned int rightCycle);
extern void motors_off();
extern void initialize_motors();

#endif // MOTOR_CONTROLLER_H
