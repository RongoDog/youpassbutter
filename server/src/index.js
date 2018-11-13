var express = require('express');
var socket = require('socket.io')
var app = express();
var port = process.env.PORT || 3000;

var server = app.listen(port, function(){
    console.log('listening to request on port 3000');
});

// Socket setup
var io = socket(server);

io.on('connection', function(socket) {
    // Clients setup
    console.log("Connection started");
    socket.on("join", function(data) {
        console.log(data, "has joined");
        socket.join(data);
    });    
    //Sending the bot commands to the Rasp_pi
    socket.on('bot-command', function(data) {
        console.log(data, "command");
        io.sockets.in('raspberry_pi').emit('bot-command', data);
    });
});
