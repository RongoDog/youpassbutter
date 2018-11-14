var express = require('express');
var socket = require('socket.io');
var path = require('path');

var app = express();
var port = process.env.PORT || 3000;


app.get('/index.html', function(req, res) {
    res.sendFile(path.join(__dirname + '/client/index.html'));
});

app.get('/js/index.js', function(req, res) {
    res.sendFile(path.join(__dirname + '/client/js/index.js'));
});

var server = app.listen(port, '0.0.0.0', function(){
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
