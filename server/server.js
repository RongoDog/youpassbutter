var express = require('express');
var socket = require('socket.io')
var app = express();
var port = process.env.PORT || 3000;

var server = app.listen(port, function(){
    console.log('listening to request on port 3000');
});

//socket setup
var io = socket(server);

io.on('connection', function(socket){
    console.log('made socket connection', socket.id);

    socket.on('bot-command', function(data){
        io.sockets.emit('bot-command', data);  // not sure wether this sends data to the pi 
    });
});