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


// emitting message on keyboard press events
var keyCode = new KeyboardEvent(event);

window.addEventListener('load', (event) => {
    event.preventDefault();
    document.addEventListener('keypress', (event) => {
        if (event.keyCode.key === "w") {
            command = {command: "forward"};
            socket.emit('bot-command', command, (data) => {
                if (data.length > 0){
                    direction.innerHTML = "moving forward";
                }
            });
    
        }else if (event.keyCode.key === 'a'){
            command = {command: "left-turn"};
            socket.emit('bot-command', command, (data) => {
                if(data.length > 0){
                    direction.innerHTML = "turning left";
                }
            });
    
        }else if (event.keyCode.key === 'd'){
            command = {command: "right-turn"};
            socket.emit('bot-command', command, (data) => {
                if(data.length > 0){
                    direction.innerHTML = "turning right";
                }
            });
    
        }else if (event.keyCode.key === 's'){
            command = {command: "backward"};
            socket.emit('bot-command', command, (data) => {
                if(data.length > 0){
                    direction.innerHTML = "moving backward";
                }
            });
        }
    })
})




// emit events on button click
/*forward.addEventListener('click', function(){
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
})*/


