/*
    Copyright (c) 2016 info@linux-projects.org
    All rights reserved.

    Redistribution and use in source and binary forms are permitted
    provided that the above copyright notice and this paragraph are
    duplicated in all such forms and that any documentation,
    advertising materials, and other materials related to such
    distribution and use acknowledge that the software was developed
    by the linux-projects.org. The name of the
    linux-projects.org may not be used to endorse or promote products derived
    from this software without specific prior written permission.
    THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
    IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
    WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE.
*/

/*
 * This is a simple "server" to be used together with UV4L which shows how to
 * process data coming from a WebRTC datachannel. The actors involved in the
 * data pipeline are the following:
 *
 * sender (e.g. browser) <--datachannel--> UV4L <--socket--> server
 *
 * More in detail, this server creates a unix domain socket of type SOCK_SEQPACKET
 * specified through command line and listens to it waiting for any incoming
 * messages. This shared socket is a bidirectional communication channel between
 * UV4L and the server. UV4L is essentially the bridge between the server and
 * the sender.
 *
 * The server acts as a very simple file server:
 * it interprets the first data packet from the open socket as the name of the file,
 * the second packet contains the size of the file in bytes, and the remaining
 * packets contains the file data until the total number bytes have been
 * transferred. On each received packet the server acknowledges the sender
 * by writing the number of bytes written to disk (or stdout).
 * The protocol is deliberately simple.
 *
 * Example:
 *     $ ./file_server /tmp/uv4l.socket /path/to/dir/
 *
 * To compile this program you need boost v1.60 or greater, for example:
 * g++ -Wall -I/path/to/boost/include/ -std=c++11 file_server.cpp -L/path/to/boost/lib -l:libboost_coroutine.a -l:libboost_context.a -l:libboost_system.a -l:libboost_thread.a -pthread -o file_server
 */

#ifndef BOOST_COROUTINES_NO_DEPRECATION_WARNING
#define BOOST_COROUTINES_NO_DEPRECATION_WARNING
#endif

#include <boost/asio/io_service.hpp>
#include <boost/asio/spawn.hpp>
#include <boost/asio/write.hpp>
#include <boost/asio/buffer.hpp>
#include <boost/asio.hpp>
#include <boost/lexical_cast.hpp>
#include <boost/optional.hpp>
#include <memory>
#include <cstdio>
#include <array>
#include <functional>
#include <iostream>
#include <fstream>

#if !defined(BOOST_ASIO_HAS_LOCAL_SOCKETS)
#error Local sockets not available on this platform.
#endif

constexpr std::size_t MAX_PACKET_SIZE = 1024 * 16;

namespace seqpacket {

    struct seqpacket_protocol {

        int type() const {
            return SOCK_SEQPACKET;
        }

        int protocol() const {
            return 0;
        }

        int family() const {
            return AF_UNIX;
        }

        using endpoint = boost::asio::local::basic_endpoint<seqpacket_protocol>;
        using socket = boost::asio::generic::seq_packet_protocol::socket;
        using acceptor = boost::asio::basic_socket_acceptor<seqpacket_protocol>;

#if !defined(BOOST_ASIO_NO_IOSTREAM)
        /// The UNIX domain iostream type.
        using iostream = boost::asio::basic_socket_iostream<seqpacket_protocol>;
#endif
    };
}

using seqpacket::seqpacket_protocol;

struct session : public std::enable_shared_from_this<session> {
    explicit session(seqpacket_protocol::socket socket) : socket_(std::move(socket)) {}

    ~session() {
        //std::cerr << "session closed\n";
    }

    void echo(boost::optional<std::string> dir, boost::asio::yield_context yield) {
        const auto self = shared_from_this();
        try {
            seqpacket_protocol::socket::message_flags in_flags = MSG_WAITALL, out_flags = MSG_WAITALL | MSG_EOR;
            // receive filename
            auto len = socket_.async_receive(boost::asio::buffer(data_), in_flags, yield);
            if (!len) // socket closed
                return;
            const std::string file{std::begin(data_), std::begin(data_) + len};
            // receive file size
            len = socket_.async_receive(boost::asio::buffer(data_), in_flags, yield);
            std::string filesize{std::begin(data_), std::begin(data_) + len};
            const auto size = boost::lexical_cast<std::size_t>(filesize);
            std::ofstream of;
            std::ostream& out = dir ? of : std::cout;
            if (dir) {
                const std::string path = *dir + '/' + file;
                of.open(path, std::ios::binary);
                std::cout << "writing to " << path << ", total bytes: " << size << '\n';
            }
            std::ios::sync_with_stdio(false);
            for (std::size_t bytes = 0; bytes < size;) {
                // Wait for the data from the client
                len = socket_.async_receive(boost::asio::buffer(data_), in_flags, yield);
                // Write the number of bytes written
                out.write(data_.data(), len);
                out.flush();
                bytes += len;
                socket_.async_send(boost::asio::buffer(std::to_string(bytes)), out_flags, yield);
            }
        } catch (const std::exception& e) {
            std::cerr << e.what() << '\n';
            socket_.close();
        }
    }

    void go(boost::optional<std::string> dir) {
        boost::asio::spawn(socket_.get_io_service(), std::bind(&session::echo, this, dir, std::placeholders::_1));
    }

private:
    seqpacket_protocol::socket socket_;
    std::array<char, MAX_PACKET_SIZE> data_;
};

int main(int argc, char* argv[]) try {
    if (argc != 2 && argc != 3) {
        std::cerr << "UV4L file server!\n"
                  << "Usage: file_server </path/to/socket> [path/to/dir>]\n"
                  << "Example: file_server /tmp/uv4l.socket /tmp/\n"
                  << "*** WARNING: existing socket file is removed ***\n"
                  << "If the directory is not specified, the file will be output to stdout\n";
        return EXIT_FAILURE;
    }

    const std::string socket{argv[1]};
    const auto dir = argc >= 3 ? boost::optional<std::string>{argv[2]} : boost::none;

    boost::asio::io_service io_service;

    std::remove(socket.c_str());

    boost::asio::spawn(io_service, [&](boost::asio::yield_context yield) {
        seqpacket_protocol::acceptor acceptor_(io_service, seqpacket_protocol::endpoint(socket));
        for (;;) {
            boost::system::error_code ec;
            seqpacket_protocol::socket socket_(io_service);
            acceptor_.async_accept(socket_, yield[ec]);
            if (!ec)
                std::make_shared<session>(std::move(socket_))->go(dir);
        }
    });

    io_service.run();

} catch (const std::exception& e) {
    std::cerr << "Exception: " << e.what() << "\n";
    return EXIT_FAILURE;
}