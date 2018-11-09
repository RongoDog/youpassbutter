#ifndef CHASSIS_H
#define CHASSIS_H

#include "drivers/motor_controller.h"

enum ChassisState {
    idle_state,
    forward_state,
    backward_state,
    left_state,
    right_state
};

enum Command {
    forward_command,
    backward_command,
    left_command,
    right_command
};

class Chassis {
    public:
        explicit Chassis();
        void give_command(Command s);

    private:
        void forward();
        void backward();
        void left();
        void right();
        void idle();

        ChassisState _current_state;
};

#endif // CHASSIS_H
