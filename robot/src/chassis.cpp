#include "chassis.h"
#include <iostream>

Chassis::Chassis() : 
    _current_state(idle_state)
{
    initialize_motors();
}

void Chassis::give_command(Command c) {
    switch(c) {
        case forward_command:
            _current_state == backward_state ? this->idle() : this->forward();
            break;
        case backward_command:
            _current_state == forward_state ? this->idle() : this->backward();
            break;
        case left_command:
            _current_state == right_state ? this->idle() : this->left();
            break;
        case right_command:
            _current_state == left_state ? this->idle() : this->right();
            break;
        default:
            std::cerr << "Invalid Command" << "\n";
    }
}

void Chassis::idle() {
    _current_state = idle_state;
    motors_off();
}

void Chassis::forward() {
    _current_state = forward_state;
    drive_forward();
}

void Chassis::backward() {
    _current_state = backward_state;
    drive_backward();
}

void Chassis::left() {
    _current_state = left_state;
    sharp_left();
}

void Chassis::right() {
    _current_state = right_state;
    sharp_right();
}
