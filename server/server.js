var express = require('express');
var socket = require('socket.io')
var app = express();
var port = process.env.PORT || 3000;

var server = app.listen(port, function(){
    console.log('listening to request on port 3000');
});

var command = "idle";
// Socket setup
var io = socket(server);

io.on('connection', function(socket){
    // Clients setup
    socket.on('join', function(data){
        socket.join(data.name);
        console.log(data.name + 'has joined');
    })
    //console.log('made socket connection', socket.id);
    
    //Sending the bot commands to the Rasp_pi
    socket.on('bot-command', function(data){
        command = data;
        io.sockets.in('raspberry_pi').emit('bot-command', data);  // not sure wether this sends data to the pi 
    });
});

app.get('/', function (req, res) {
  res.send(command);
})
