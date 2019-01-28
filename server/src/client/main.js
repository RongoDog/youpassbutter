(function () {
    var socket = io.connect('http://smokesong.xyz:3000');
    socket.emit("join", "client");
    var signalObj = null;

    window.addEventListener('DOMContentLoaded', function () {
        var isStreaming = false;
        var video = document.getElementById('v');
        var start = document.getElementById('start');
        var stop = document.getElementById('stop');
        var accelDataX = [];
        var accelDataY = [];
        var accelDataZ = [];
        var rotateDataX = [];
        var rotateDataY = [];
        var rotateDataZ = [];
        var accelTime = [];
        graphDivAccelX = document.getElementById('plotAccelX');
        graphDivAccelY = document.getElementById('plotAccelY');
        graphDivAccelZ = document.getElementById('plotAccelZ');
        graphDivRotateX = document.getElementById('plotRotateX');
        graphDivRotateY = document.getElementById('plotRotateY');
        graphDivRotateZ = document.getElementById('plotRotateZ')
        Plotly.newPlot(graphDivAccelX, 
            [{ 
                x: [],
                y: [], 
            }],
            {
                margin: { t: 0 } 
            }
        );
        Plotly.newPlot(graphDivAccelY, 
            [{ 
                x: [],
                y: [], 
            }],
            {
                margin: { t: 0 } 
            }
        );
        Plotly.newPlot(graphDivAccelZ, 
            [{ 
                x: [],
                y: [], 
            }],
            {
                margin: { t: 0 } 
            }
        );
        Plotly.newPlot(graphDivRotateX, 
            [{ 
                x: [],
                y: [], 
            }],
            {
                margin: { t: 0 } 
            }
        );
        Plotly.newPlot(graphDivRotateY, 
            [{ 
                x: [],
                y: [], 
            }],
            {
                margin: { t: 0 } 
            }
        );
        Plotly.newPlot(graphDivRotateZ, 
            [{ 
                x: [],
                y: [], 
            }],
            {
                margin: { t: 0 } 
            }
        );

        start.addEventListener('click', function (e) {
            if (!isStreaming) {
                signalObj = new signal(socket,
                    function (stream) {
                        console.log('got a stream!');
                        //var url = window.URL || window.webkitURL;
                        //video.src = url ? url.createObjectURL(stream) : stream; // deprecated
                        video.srcObject = stream;
                        video.play();
                    },
                    function (error) {
                        alert(error);
                    },
                    function () {
                        ctx.clearRect(0, 0, canvas.width, canvas.height);
                        console.log('websocket closed. bye bye!');
                        video.srcObject = null;
                        isStreaming = false;
                    },
                    function (message) {
                        alert(message);
                    },
                    function (data) {
                        var raw =  window.atob(data);
                        function conversion(data, i) {
                            var combined = (data.charCodeAt(i) << 8) + data.charCodeAt(i+1);	
                            var negative = (combined & (1 << 15)) != 0;	
                            var nativeInt;	
                            if (negative) {	
                                nativeInt = combined | ~((1 << 16) - 1);	
                            } else {	
                                nativeInt = combined;	
                            }
                            return nativeInt;
                        }
                        for (i = 0; i < raw.length; i+=2) {
                          const dataType = i%12;
                          switch(dataType) {
                            case 0:
                                accelDataX.push(conversion(raw, i)/8192.0);
                                accelTime.push(accelDataX.length + 1);
                                break;
                            case 1:
                                accelDataY.push(conversion(raw, i)/8192.0);
                                break;
                            case 2:
                                accelDataZ.push(conversion(raw, i)/8192.0);
                                break;
                            case 3:
                                rotateDataX.push(conversion(raw, i)/131.0);
                                break;
                            case 4:
                                rotateDataY.push(conversion(raw, i)/131.0);
                                break;
                            case 5:
                                rotateDataZ.push(conversion(raw, i)/131.0);
                                break;
                            default:
                          }
                        }
                        if (accelDataX.length > 50) {
                            Plotly.deleteTraces(graphDivAccelX, 0);
                            Plotly.deleteTraces(graphDivAccelY, 0);
                            Plotly.deleteTraces(graphDivAccelZ, 0);
                            Plotly.deleteTraces(graphDivRotateX, 0);
                            Plotly.deleteTraces(graphDivRotateY, 0);
                            Plotly.deleteTraces(graphDivRotateZ, 0);
                            Plotly.plot(graphDivAccelX, [{
                                x: accelTime.slice(0, 49),
                                y: accelDataX.slice(accelDataX.length - 51, accelDataX.length - 1) }], 
                                { margin: { t: 0 }, yaxis: { range: [-4, 4], } }
                            );
                            Plotly.plot(graphDivAccelY, [{
                                x: accelTime.slice(0, 49),
                                y: accelDataY.slice(accelDataY.length - 51, accelDataY.length - 1) }], 
                                { margin: { t: 0 }, yaxis: { range: [-4, 4], } }
                            );
                            Plotly.plot(graphDivAccelZ, [{
                                x: accelTime.slice(0, 49),
                                y: accelDataY.slice(accelDataZ.length - 51, accelDataZ.length - 1) }], 
                                { margin: { t: 0 }, yaxis: { range: [-4, 4], } }
                            );
                            Plotly.plot(graphDivRotateX, [{
                                x: accelTime.slice(0, 49),
                                y: rotateDataX.slice(rotateDataX.length - 51, rotateDataX.length - 1) }], 
                                { margin: { t: 0 }, yaxis: { range: [-4, 4], } }
                            );
                            Plotly.plot(graphDivRotateY, [{
                                x: accelTime.slice(0, 49),
                                y: rotateDataY.slice(rotateDataY.length - 51, rotateDataY.length - 1) }], 
                                { margin: { t: 0 }, yaxis: { range: [-4, 4], } }
                            );
                            Plotly.plot(graphDivRotateZ, [{
                                x: accelTime.slice(0, 49),
                                y: rotateDataZ.slice(rotateDataZ.length - 51, rotateDataZ.length - 1) }], 
                                { margin: { t: 0 }, yaxis: { range: [-4, 4], } }
                            );

                        } 
                    }
                );
            }
        }, false);

        stop.addEventListener('click', function (e) {
            if (signalObj) {
                signalObj.hangup();
                signalObj = null;
            }
        }, false);
        socket.on("webrtc-relay", (data) => {
            signalObj.onMessage(JSON.parse(data));
        });

        // Wait until the video stream can play
        video.addEventListener('canplay', function (e) {
            if (!isStreaming) {
                isStreaming = true;
            }
        }, false);

        // Wait for the video to start to play
        video.addEventListener('play', function () {
            // Every 33 milliseconds copy the video image to the canvas
            setInterval(function () {
                if (video.paused || video.ended) {
                    return;
                }
            }, 1000000);
        }, false);
        renderVisualization();
    });
})();