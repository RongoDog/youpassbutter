#include <iostream>
#include "socket_handler.h"
#include "chassis.h"
#include <stdio.h>
#include <stdlib.h>

#define BIND_EVENT(IO,EV,FN) \
    do{ \
        socket::event_listener_aux l = FN;\
        IO->on(EV,l);\
    } while(0)

SocketHandler::SocketHandler(Chassis *chassis) :
    _io(new client()),
    _chassis(chassis)
{
    using std::placeholders::_1;
    using std::placeholders::_2;
    using std::placeholders::_3;
    using std::placeholders::_4;
    socket::ptr sock = _io->socket();
    BIND_EVENT(sock,"bot-command", std::bind(&SocketHandler::on_command,this,_1,_2,_3,_4));
    _io->set_socket_open_listener(std::bind(&SocketHandler::on_connected,this,_1));
    _io->connect("http://smokesong.xyz:3000");
}

void SocketHandler::on_connected(std::string const& nsp)
{
    _io->socket(nsp)->emit("join", string_message::create("raspberry_pi"));
}

void SocketHandler::on_command(std::string const& name,message::ptr const& data,bool hasAck,message::list &ack_resp) {
    std::string command_string = data->get_string();
    std::cout << command_string << "\n";
    if (command_string == "forward") {
        _chassis->give_command(forward_command);
    } else if (command_string == "backward") {
        _chassis->give_command(backward_command);
    } else if (command_string == "left") {
        _chassis->give_command(left_command);
    } else if (command_string == "right") {
        _chassis->give_command(right_command);
    } else if (command_string == "turn-off") {
        system("shutdown -P now");
    } else {
        std::cerr << "Invalid Command\n";
    }
}