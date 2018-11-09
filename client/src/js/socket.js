// making connection to server 
var socket = io.connect('http://localhost:3000');
socket.emit('join', {name: 'client'});

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
    console.log("clicked");
    command = {command: "move forward"};

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
})*/


