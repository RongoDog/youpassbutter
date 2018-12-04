#include <iostream>
#include <stdio.h>
#include <stdlib.h>
#include "socket_handler.h"

#include <websocketpp/config/asio_no_tls.hpp>
#include <websocketpp/server.hpp>

typedef websocketpp::server<websocketpp::config::asio> server;

using websocketpp::connection_hdl;
using websocketpp::lib::placeholders::_1;
using websocketpp::lib::placeholders::_2;
using websocketpp::lib::bind;

#define BIND_EVENT(IO,EV,FN) \
    do{ \
        socket::event_listener_aux l = FN;\
        IO->on(EV,l);\
    } while(0)

SocketHandler::SocketHandler(Chassis *chassis) :
    _io(new client()),
    _chassis(chassis)
    //_webrtc_socket(websocket_client client)
{
    //_client.connect("wss://localhost:8080").wait();
    using std::placeholders::_1;
    using std::placeholders::_2;
    using std::placeholders::_3;
    using std::placeholders::_4;
    socket::ptr sock = _io->socket();
    BIND_EVENT(sock,"bot-command", std::bind(&SocketHandler::on_command,this,_1,_2,_3,_4));
    //BIND_EVENT(sock,"webrtc-relay", std::bind(&SocketHandler::on_webrtc_relay,this,_1,_2,_3,_4));
    _io->set_socket_open_listener(std::bind(&SocketHandler::on_connected,this,_1));
    _io->connect("http://smokesong.xyz:3000");
}

void SocketHandler::on_connected(std::string const& nsp)
{
    _io->socket(nsp)->emit("join", string_message::create("raspberry_pi"));
}

void SocketHandler::on_command(std::string const& name,message::ptr const& data,bool hasAck,message::list &ack_resp) {
    std::string command_string = data->get_string();
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
/**
void SocketHandler::on_webrtc_relay(std::string const& name,message::ptr const& data,bool hasAck,message::list &ack_resp) {
    std::string stringified_json = data->get_string();
    websocket_outgoing_message out_msg;
    out_msg.set_utf8_message(stringified_json);
    _webrtc_socket->send(out_msg).wait();
    client.receive().then([](websocket_incoming_message in_msg) {
      return in_msg.extract_string();
    }).then([](string body) {
      cout << body << endl; // test
    }).wait();
}
**/