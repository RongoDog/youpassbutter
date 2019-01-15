#include <stdio.h>
#include <stdlib.h>
#include "communications.h"
#include "chassis.h"

using websocketpp::connection_hdl;
using std::placeholders::_1;
using std::placeholders::_2;
using std::placeholders::_3;
using std::placeholders::_4;

typedef websocketpp::client<websocketpp::config::asio_client> websocket_client;

#define BIND_EVENT(IO,EV,FN) \
    do{ \
        socket::event_listener_aux l = FN;\
        IO->on(EV,l);\
    } while(0)

CommunicationsModule::CommunicationsModule(Chassis *chassis) :
    _io(new client()),
    _c(new websocket_client()),
    _chassis(chassis)
{
    // The following describe the socket/io communication with the server
    socket::ptr sock = _io->socket();
    BIND_EVENT(sock,"bot-command", std::bind(&CommunicationsModule::on_command,this,_1,_2,_3,_4));
    BIND_EVENT(sock,"webrtc-relay", std::bind(&CommunicationsModule::on_webrtc_relay,this,_1,_2,_3,_4));
    _io->set_socket_open_listener(std::bind(&CommunicationsModule::on_connected,this,_1));
    _io->connect("http://smokesong.xyz:3000");

    // The following describes the uv4l websocket
    std::string uri = "ws://localhost:8080";
    try {
        // Set logging to be pretty verbose (everything except message payloads)
        _c->set_access_channels(websocketpp::log::alevel::all);
        _c->clear_access_channels(websocketpp::log::alevel::frame_payload);

        // Initialize ASIO
        _c->init_asio();
        // Register our message handler
        _c->set_message_handler(
            websocketpp::lib::bind(
                &CommunicationsModule::on_uv4l_message,
                this,
                ::_1,
                ::_2
            )
        );
        _c->set_open_handler(
            websocketpp::lib::bind(
                &CommunicationsModule::on_uv4l_open,
                this,
                ::_1
            )
        );
        _c->set_close_handler(
            websocketpp::lib::bind(
                &CommunicationsModule::on_uv4l_close,
                this,
                ::_1
            )
        );
        _c->set_fail_handler(
            websocketpp::lib::bind(
                &CommunicationsModule::on_uv4l_fail,
                this,
                websocketpp::lib::placeholders::_1
            )
        );
        websocketpp::lib::error_code ec;
        websocket_client::connection_ptr con = _c->get_connection(uri, ec);
        if (ec) {
            std::cout << "could not create connection because: " << ec.message() << std::endl;
            exit(1);
        }
        // Grab a handle for this connection so we can talk to it in a thread 
        // safe manor after the event loop starts.
        m_hdl = con->get_handle();

        // Note that connect here only requests a connection. No network messages are
        // exchanged until the event loop starts running in the next line.
        _c->connect(con);

        // Start the ASIO io_service run loop
        // this will cause a single connection to be made to the server. c.run()
        // will exit when this connection is closed.
        _c->run();
    } catch (websocketpp::exception const & e) {
        std::cout << e.what() << std::endl;
    }
}

/**
* Once we're connection, we save the 
*/
void CommunicationsModule::on_connected(std::string const& nsp)
{
    _io->socket()->emit("join", string_message::create("raspberry_pi"));
}

/**
* The following function is the manual command interface into the robot.
*/
void CommunicationsModule::on_command(std::string const& name,message::ptr const& data,bool hasAck,message::list &ack_resp) {
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

void CommunicationsModule::on_webrtc_relay(std::string const& name,message::ptr const& data,bool hasAck,message::list &ack_resp) {
    std::string stringified_json = data->get_string();
    websocketpp::lib::error_code ec;
    _c->send(m_hdl, stringified_json, websocketpp::frame::opcode::text,ec);;
    // The most likely error that we will get is that the connection is
    // not in the right state. Usually this means we tried to send a 
    // message to a connection that was closed or in the process of 
    // closing. While many errors here can be easily recovered from, 
    // in this simple example, we'll stop the telemetry loop.
    if (ec) {
        _c->get_alog().write(websocketpp::log::alevel::app, 
        	  "Send Error: "+ec.message());
    }
}

void CommunicationsModule::on_uv4l_message(websocketpp::connection_hdl hdl, message_ptr msg) {
    _c->get_alog().write(websocketpp::log::alevel::app, "Received Reply: "+msg->get_payload());
    _io->socket()->emit("webrtc-relay", string_message::create(msg->get_payload()));
}

void CommunicationsModule::on_uv4l_open(websocketpp::connection_hdl hdl) {
    _c->get_alog().write(websocketpp::log::alevel::app, 
        "Connection opened, starting signaling");
}

void CommunicationsModule::on_uv4l_close(websocketpp::connection_hdl hdl) {
    _c->get_alog().write(websocketpp::log::alevel::app, 
        "Connection closed, stopping signaling!");
}

void CommunicationsModule::on_uv4l_fail(websocketpp::connection_hdl hdl) {
    _c->get_alog().write(websocketpp::log::alevel::app, 
        "Connection failed, stopping signaling!");
}