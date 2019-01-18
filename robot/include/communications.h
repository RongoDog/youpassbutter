#ifndef COMMUNICATIONS_H
#define COMMUNICATIONS_H

#include <websocketpp/client.hpp>
#include <websocketpp/config/asio_no_tls_client.hpp>
#include <sio_client.h>
#include <iostream>

using namespace std;

typedef websocketpp::client<websocketpp::config::asio_client> websocket_client;
typedef websocketpp::config::asio_client::message_type::ptr message_ptr;

class Chassis;
class CommunicationsModule {
    public:
        explicit CommunicationsModule(Chassis *chassis);
    private:
        void on_connected(std::string const& nsp);
        void on_command(std::string const& name, sio::message::ptr const& data, bool hasAck, sio::message::list &ack_resp);
        void on_webrtc_relay(std::string const& name, sio::message::ptr const& data, bool hasAck, sio::message::list &ack_resp);

        void on_uv4l_open(websocketpp::connection_hdl hdl);
        void on_uv4l_fail(websocketpp::connection_hdl hdl);
        void on_uv4l_close(websocketpp::connection_hdl hdl);
        void on_uv4l_message(websocketpp::connection_hdl hdl, message_ptr msg);

        std::unique_ptr<websocket_client> _c;
        std::unique_ptr<sio::client> _io;
        std::unique_ptr<Chassis> _chassis;
        websocketpp::connection_hdl m_hdl;
};

#endif // COMMUNICATIONS_H
