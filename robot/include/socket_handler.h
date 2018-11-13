#ifndef SOCKETHANDLER_H
#define SOCKETHANDLER_H

#include <sio_client.h>
#include "chassis.h"

using namespace sio;

class SocketHandler {
    public:
        explicit SocketHandler(Chassis *chassis);

    private:
        void on_connected(std::string const& nsp);
        void on_command(std::string const& name,message::ptr const& data,bool hasAck,message::list &ack_resp);

        std::unique_ptr<client> _io;
        std::unique_ptr<Chassis> _chassis;
};

#endif // SOCKETHANDLER_H
