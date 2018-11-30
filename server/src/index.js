var express = require("express");
var socket = require("socket.io");
var path = require("path");

var app = express();
var port = process.env.PORT || 3000;


app.get("/index.html", function(req, res) {
    res.sendFile(path.join(__dirname + "/client/index.html"));
});

app.get("/js/index.js", function(req, res) {
    res.sendFile(path.join(__dirname + "/client/js/index.js"));
});

var server = app.listen(port, "0.0.0.0", function(){
    console.log("listening to request on port 3000");
});

// Socket setup
var io = socket(server);
var connectionState = {
    raspberryPiConnected: false,
    clientConnected: false,
};

function failedValidation(requiredId, id) {
    return id !== requiredId;
}

io.on("connection", (socket) => {
    var id;

    //The join signal allows us to begin communication and
    //enable authentication.
    socket.on("join", (data) => {
        if (data.id === "raspberry_pi" && 
            data.password === process.env.RASPI_PASS
        ) {
            socket.join("raspberry_pi");
            connectionState.raspberryPiConnected = true;
            id = "raspberry_pi";
            io.sockets.in("client").emit("is_pi_connected", true)
        }
        else if (data.id === "client" &&
            data.password === process.env.CLIENT_PASS &&
            connectionState.clientConnected === false
        ) {
            socket.join("client");
            id = "client";
            socket.emit("is_pi_connected", connectionState.raspberryPiConnected);
        } else {
            socket.emit("disconnect");
        }
    });    
    //Sending the bot commands to the Raspberry Pi
    socket.on("bot-command", (data) => {
        if (failedValidation("client", id)) {
            socket.emit("disconnect");
        }
        io.sockets.in("raspberry_pi").emit("bot-command", data);
    });


    //We disconnect from the state
    socket.on("disconnect", () => {
        if (id === "raspberry_pi") {
            connectionState.raspberryPiConnected = false;
            io.sockets.in("client").emit("is_pi_connected", false)
        } else {
            connectionState.clientConnected = false;
        }
    });
});



