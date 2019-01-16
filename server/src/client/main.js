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

      window.addEventListener("load", (e) => {
        document.addEventListener("keypress", (event) => {
            event.preventDefault();
            switch (event.code) {
                case "KeyW":
                    socket.emit('bot-command', "forward");
                    break;
                case "KeyA":
                    socket.emit('bot-command', "left");
                    break;
                case "KeyD":
                    socket.emit('bot-command', "right");
                    break;
                case "KeyS":
                    socket.emit('bot-command', "backward");
                    break;
                case "KeyO":
                    socket.emit('bot-command', "turn-off");
                    break;
                default:
                    break;
            }
        });
    });
    renderVisualization();
  });
})();