(function () {
    var socket = io.connect('http://smokesong.xyz:3000');
    socket.emit("join", "client");
    var signalObj = null;

    window.addEventListener('DOMContentLoaded', function () {
        var isStreaming = false;
        var video = document.getElementById('v');
        var start = document.getElementById('start');
        var stop = document.getElementById('stop');
        var accelData = [];
        var accelTime = [];
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
                        console.log(data);
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
                                accelData.push(conversion(raw, i)/8192.0);
                                accelTime.push(accelData.length + 1);
                            case 1:
                            case 2:
                            case 3:
                            case 4:
                            case 5:
                            default:
                          }
                          PLOT = document.getElementById('plot');
                          Plotly.plot( PLOT, [{
                          x: accelTime,
                          y: accelData }], {
                          margin: { t: 0 } } );
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
                canvas.setAttribute('width', video.videoWidth);
                canvas.setAttribute('height', video.videoHeight);
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
                var w = canvas.getAttribute('width');
                var h = canvas.getAttribute('height');
                ctx.fillRect(0, 0, w, h);
                ctx.drawImage(video, 0, 0, w, h);
            }, 33);
        }, false);
        renderVisualization();
    });
})();