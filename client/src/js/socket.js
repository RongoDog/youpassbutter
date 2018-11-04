// making connection to server 
var socket = io.connect('http://smokey.xyz');

// query dom
var forward = document.getElementById('forward');
var left = document.getElementById('left');
var right = document.getElementById('right');
var reverse = document.getElementById('reverse');
var direction = document.getElementById('current-bot-direction');
var command;

forward.value = "forward";
left.value = "left";
right.value = "right";
reverse.value = "reverse";

// emit events on button press
forward.addEventListener('click', function(){
    command = {command: "move forward"};
    socket.emit('bot-command', command, (data) => {
        if (data.length > 0){
            direction.innerHTML = "moving forward";
        }
    });
})

left.addEventListener('click', function(){
    command = {command: "turn left"};
    socket.emit('bot-command', command, (data) => {
        if(data.length > 0){
            direction.innerHTML = "turning left";
        }
    });
})

right.addEventListener('click', function(){
    command = {command: "turn right"};
    socket.emit('bot-command', command, (data) => {
        if(data.length > 0){
            direction.innerHTML = "turning right";
        }
    });
})

reverse.addEventListener('click', function(){
    command = {command: "move backward"};
    socket.emit('bot-command', command, (data) => {
        if(data.length > 0){
            direction.innerHTML = "moving backward";
        }
    });
})

// listen for events
socket.on('bot-co')

