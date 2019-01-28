/*
 * window.mozRTCPeerConnection, window.mozRTCSessionDescription, window.mozRTCIceCandidate are now deprecated
 */

RTCPeerConnection = window.RTCPeerConnection || /*window.mozRTCPeerConnection ||*/ window.webkitRTCPeerConnection;
RTCSessionDescription = /*window.mozRTCSessionDescription ||*/ window.RTCSessionDescription;
RTCIceCandidate = /*window.mozRTCIceCandidate ||*/ window.RTCIceCandidate;

function signal(socket, onStream, onError, onClose, onMessage, onData) {
  var pc;
  var iceCandidates = [];
  var hasRemoteDesc = false;

  function addIceCandidates() {
    if (hasRemoteDesc) {
      iceCandidates.forEach(function (candidate) {
        pc.addIceCandidate(candidate,
          function () {
            console.log("IceCandidate added: " + JSON.stringify(candidate));
          },
          function (error) {
            console.error("addIceCandidate error: " + error);
          }
        );
      });
      iceCandidates = [];
    }
  }
  /* First we create a peer connection */
  var config = { "iceServers": [{ "urls": ["stun:stun.l.google.com:19302"] }] };
  var options = { optional: [] };
  pc = new RTCPeerConnection(config, options);
  iceCandidates = [];
  hasRemoteDesc = false;

  pc.onicecandidate = function (event) {
    if (event.candidate) {
      var candidate = {
        sdpMLineIndex: event.candidate.sdpMLineIndex,
        sdpMid: event.candidate.sdpMid,
        candidate: event.candidate.candidate
      };
      var request = {
        what: "addIceCandidate",
        data: JSON.stringify(candidate)
      };
      socket.emit("webrtc-relay", JSON.stringify(request));
    } else {
      console.log("end of candidates.");
    }
  };

  if ('ontrack' in pc) {
    pc.ontrack = function (event) {
      onStream(event.streams[0]);
    };
  } else {  // onaddstream() deprecated
    pc.onaddstream = function (event) {
      onStream(event.stream);
    };
  }

  pc.onremovestream = function (event) {
    console.log("the stream has been removed: do your stuff now");
  };

  pc.ondatachannel = function (event) {
    setupKeyCommands(event.channel);
    event.channel.onopen = () => console.log("Data Channel opened");
    event.channel.onerror = (err) => console.error("Data Channel Error:", err);
    event.channel.onmessage = (event) => {
      onData(event.data);
    };
    event.channel.onclose = () => console.log("The Data Channel is Closed");
  };

  /* kindly signal the remote peer that we would like to initiate a call */
  var request = {
    what: "call",
    options: {
      // If forced, the hardware codec depends on the arch.
      // (e.g. it's H264 on the Raspberry Pi)
      // Make sure the browser supports the codec too.
      force_hw_vcodec: true,
      vformat: 30, /* 30=640x480, 30 fps */
      trickle_ice: true
    }
  };
  socket.emit("webrtc-relay", JSON.stringify(request));

  this.onMessage = function(msg) {
    var what = msg.what;
    var data = msg.data;
    switch (what) {
      case "offer":
        var mediaConstraints = {
          optional: [],
          mandatory: {
            OfferToReceiveAudio: false,
            OfferToReceiveVideo: true
          }
        };
        pc.setRemoteDescription(new RTCSessionDescription(JSON.parse(data)),
          function onRemoteSdpSuccess() {
            hasRemoteDesc = true;
            addIceCandidates();
            pc.createAnswer(function (sessionDescription) {
              pc.setLocalDescription(sessionDescription);
              var request = {
                what: "answer",
                data: JSON.stringify(sessionDescription)
              };
              socket.emit("webrtc-relay", JSON.stringify(request));
            }, function (error) {
              onError("failed to create answer: " + error);
            }, mediaConstraints);
          },
          function onRemoteSdpError(event) {
            onError('failed to set the remote description: ' + event);
          }
        );
        break;

      case "answer":
        break;

      case "message":
        if (onMessage) {
          onMessage(msg.data);
        }
        break;

      case "iceCandidate": // received when trickle ice is used (see the "call" request)
        if (!msg.data) {
          console.log("Ice Gathering Complete");
          break;
        }
        var elt = JSON.parse(msg.data);
        let candidate = new RTCIceCandidate({ sdpMLineIndex: elt.sdpMLineIndex, candidate: elt.candidate });
        iceCandidates.push(candidate);
        addIceCandidates(); // it internally checks if the remote description has been set
        break;

      case "iceCandidates": // received when trickle ice is NOT used (see the "call" request)
        var candidates = JSON.parse(msg.data);
        for (var i = 0; candidates && i < candidates.length; i++) {
          var elt = candidates[i];
          let candidate = new RTCIceCandidate({ sdpMLineIndex: elt.sdpMLineIndex, candidate: elt.candidate });
          iceCandidates.push(candidate);
        }
        addIceCandidates();
        break;
    }
  };

  this.hangup = function () {
    var request = {
      what: "hangup"
    };
    console.log("send message " + JSON.stringify(request));
    socket.emit("webrtc-relay", JSON.stringify(request));
  };
}
