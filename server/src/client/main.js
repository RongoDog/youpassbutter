(function () {
    var socket = io.connect('http://smokesong.xyz:3000');
    socket.emit("join", "client");
    var signalObj = null;

    window.addEventListener('DOMContentLoaded', function () {
        var isStreaming = false;
        var video = document.getElementById('v');
        var canvas = document.getElementById('c');
        var ctx = canvas.getContext('2d');
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
                        var raw =  window.atob(data);
                        var HEX = '';
                        function conversion(byteH, byteL) {
                            combined = (byteH << 8) + byteL;	
                            negative = (combined & (1 << 15)) != 0;	
                            nativeInt;	
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
                                console.log(conversion(raw[i], raw[i+1])/8192.0);
                            case 1:
                            case 2:
                            case 3:
                            case 4:
                            case 5:
                            default:
                          }
                          var _hex = raw.charCodeAt(i).toString(16)
                      
                          HEX += (_hex.length==2?_hex:'0'+_hex);
                      
                        }
                        console.log(HEX.toUpperCase());
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