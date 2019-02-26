(function () {
    var socket = io.connect('http://smokesong.xyz:3000');
    socket.emit("join", "client");
    var signalObj = null;
    var dataChannel;
    window.addEventListener('DOMContentLoaded', function () {
        var isStreaming = false;
        var video = document.getElementById('v');
        var start = document.getElementById('start');
        var stop = document.getElementById('stop');
        var fd = document.getElementById('fd');

        let videoWidth, videoHeight;
        let canvasOutput = document.getElementsByTagName('canvas')[0];
        let canvasOutputCtx = canvasOutput.getContext('2d');
        let faceClassifier = null;
        let src = null;
        let dstC1 = null;
        let canvasInput = null;
        let canvasInputCtx = null;
        let canvasBuffer = null;
        let canvasBufferCtx = null;

        var accelDataX = [];
        var accelDataY = [];
        var accelDataZ = [];
        document.addEventListener("keypress", (event) => {
            event.preventDefault();
            switch (event.code) {
                case "KeyP":
                    console.log(accelDataY);
                    break;
                default:
                    break;
            }
        });
        var rotateDataX = [];
        var rotateDataY = [];
        var rotateDataZ = [];
        var accelTime = [];
        //graphDivAccelX = document.getElementById('plotAccelX');
        graphDivAccelY = document.getElementById('plotAccelY');
        //graphDivAccelZ = document.getElementById('plotAccelZ');
        //graphDivRotateX = document.getElementById('plotRotateX');
        //graphDivRotateY = document.getElementById('plotRotateY');
        //graphDivRotateZ = document.getElementById('plotRotateZ')
        //Plotly.newPlot(graphDivAccelX, 
        //    [{ 
        //        x: [],
        //        y: [], 
        //    }],
        //    {
        //        margin: { t: 0 } 
        //    }
        //);
        Plotly.newPlot(graphDivAccelY,
            [{
                x: [],
                y: [],
            }],
            {
                margin: { t: 0 }
            }
        );
        //Plotly.newPlot(graphDivAccelZ, 
        //    [{ 
        //        x: [],
        //        y: [], 
        //    }],
        //    {
        //        margin: { t: 0 } 
        //    }
        //);
        //Plotly.newPlot(graphDivRotateX, 
        //    [{ 
        //        x: [],
        //        y: [], 
        //    }],
        //    {
        //        margin: { t: 0 } 
        //    }
        //);
        //Plotly.newPlot(graphDivRotateY, 
        //    [{ 
        //        x: [],
        //        y: [], 
        //    }],
        //    {
        //        margin: { t: 0 } 
        //    }
        //);
        //Plotly.newPlot(graphDivRotateZ, 
        //    [{ 
        //        x: [],
        //        y: [], 
        //    }],
        //    {
        //        margin: { t: 0 } 
        //    }
        //);

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
                        var raw = window.atob(data);
                        function conversion(data, i) {
                            var combined = (data.charCodeAt(i) << 8) + data.charCodeAt(i + 1);
                            var negative = (combined & (1 << 15)) != 0;
                            var nativeInt;
                            if (negative) {
                                nativeInt = combined | ~((1 << 16) - 1);
                            } else {
                                nativeInt = combined;
                            }
                            return nativeInt;
                        }
                        for (i = 0; i < raw.length; i += 2) {
                            const dataType = i % 6;
                            switch (dataType) {
                                case (0):
                                    accelDataX.push(conversion(raw, i) / 8192.0);
                                    accelTime.push(accelDataX.length + 1);
                                    break;
                                case (2):
                                    accelDataY.push(conversion(raw, i) / 8192.0);
                                    break;
                                case (4):
                                    accelDataZ.push(conversion(raw, i) / 8192.0);
                                    break;
                                default:
                            }
                        }
                        if (accelDataX.length > 50) {
                            //Plotly.deleteTraces(graphDivAccelX, 0);
                            Plotly.deleteTraces(graphDivAccelY, 0);
                            //Plotly.deleteTraces(graphDivAccelZ, 0);
                            //Plotly.deleteTraces(graphDivRotateX, 0);
                            //Plotly.deleteTraces(graphDivRotateY, 0);
                            //Plotly.deleteTraces(graphDivRotateZ, 0);
                            //Plotly.plot(graphDivAccelX, [{
                            //    x: accelTime.slice(0, 49),
                            //    y: accelDataX.slice(accelDataX.length - 51, accelDataX.length - 1) }], 
                            //    { margin: { t: 0 }, yaxis: { range: [-4, 4], } }
                            //);
                            Plotly.plot(graphDivAccelY, [{
                                x: accelTime.slice(0, 99),
                                y: accelDataY.slice(accelDataY.length - 99, accelDataY.length - 1)
                            }],
                                { margin: { t: 0 }, yaxis: { range: [-4, 4], } }
                            );
                            //Plotly.plot(graphDivAccelZ, [{
                            //    x: accelTime.slice(0, 49),
                            //    y: accelDataY.slice(accelDataZ.length - 51, accelDataZ.length - 1) }], 
                            //    { margin: { t: 0 }, yaxis: { range: [-4, 4], } }
                            //);
                            //Plotly.plot(graphDivRotateX, [{
                            //    x: accelTime.slice(0, 49),
                            //    y: rotateDataX.slice(rotateDataX.length - 51, rotateDataX.length - 1) }], 
                            //    { margin: { t: 0 }, yaxis: { range: [-250, 250], } }
                            //);
                            //Plotly.plot(graphDivRotateY, [{
                            //    x: accelTime.slice(0, 49),
                            //    y: rotateDataY.slice(rotateDataY.length - 51, rotateDataY.length - 1) }], 
                            //    { margin: { t: 0 }, yaxis: { range: [-250, 250], } }
                            //);
                            //Plotly.plot(graphDivRotateZ, [{
                            //    x: accelTime.slice(0, 49),
                            //    y: rotateDataZ.slice(rotateDataZ.length - 51, rotateDataZ.length - 1) }], 
                            //    { margin: { t: 0 }, yaxis: { range: [-250, 250], } }
                            //);

                        }
                    },
                    function (newDataChannel) {
                        dataChannel = newDataChannel;
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
                videoWidth = video.videoWidth;
                videoHeight = video.videoHeight;
                video.setAttribute("width", videoWidth);
                video.setAttribute("height", videoHeight);
                canvasOutput.width = videoWidth;
                canvasOutput.height = videoHeight;
                isStreaming = true;
            }
            startVideoProcessing();
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

        function startVideoProcessing() {
            stopVideoProcessing();
            canvasInput = document.createElement('canvas');
            canvasInput.width = videoWidth;
            canvasInput.height = videoHeight;
            canvasInputCtx = canvasInput.getContext('2d');

            canvasBuffer = document.createElement('canvas');
            canvasBuffer.width = videoWidth;
            canvasBuffer.height = videoHeight;
            canvasBufferCtx = canvasBuffer.getContext('2d');

            srcMat = new cv.Mat(videoHeight, videoWidth, cv.CV_8UC4);
            grayMat = new cv.Mat(videoHeight, videoWidth, cv.CV_8UC1);

            faceClassifier = new cv.CascadeClassifier();
            faceClassifier.load('haarcascade_frontalface_default.xml');

            requestAnimationFrame(processVideo);
        }
        function processVideo() {
            canvasInputCtx.drawImage(video, 0, 0, videoWidth, videoHeight);
            let imageData = canvasInputCtx.getImageData(0, 0, videoWidth, videoHeight);
            srcMat.data.set(imageData.data);
            cv.cvtColor(srcMat, grayMat, cv.COLOR_RGBA2GRAY);
            let faces = [];
            let size;
            let faceVect = new cv.RectVector();
            let faceMat = new cv.Mat();
            cv.pyrDown(grayMat, faceMat);
            cv.pyrDown(faceMat, faceMat);
            size = faceMat.size();
            faceClassifier.detectMultiScale(faceMat, faceVect);
            for (let i = 0; i < faceVect.size(); i++) {
                let face = faceVect.get(i);
                if (fd.checked) {
                    if (face.x > 55) {
                        byteArray[0] = 3;
                        if (face.x > 80) {
                            byteArray[1] = 255;
                        } else {
                            byteArray[1] = 200;
                        }
                        dataChannel.send(byteArray);
                        byteArray[0] = 2;
                        byteArray[1] = 3;
                        dataChannel.send(byteArray);
                    } else if (face.x < 55) {
                        byteArray[0] = 3;
                        if (face.x < 20) {
                            byteArray[1] = 255;
                        } else {
                            byteArray[1] = 200;
                        }
                        dataChannel.send(byteArray);
                        byteArray[0] = 2;
                        byteArray[1] = 4;
                        dataChannel.send(byteArray);
                    }
                }
                faces.push(new cv.Rect(face.x, face.y, face.width, face.height));
            }
            faceMat.delete();
            faceVect.delete();

            canvasOutputCtx.drawImage(canvasInput, 0, 0, videoWidth, videoHeight);
            drawResults(canvasOutputCtx, faces, 'red', size);
            requestAnimationFrame(processVideo);
        }

        function drawResults(ctx, results, color, size) {
            for (let i = 0; i < results.length; ++i) {
                let rect = results[i];
                let xRatio = videoWidth / size.width;
                let yRatio = videoHeight / size.height;
                ctx.lineWidth = 3;
                ctx.strokeStyle = color;
                ctx.strokeRect(rect.x * xRatio, rect.y * yRatio, rect.width * xRatio, rect.height * yRatio);
            }
        }
        function stopVideoProcessing() {
            if (src != null && !src.isDeleted()) src.delete();
            if (dstC1 != null && !dstC1.isDeleted()) dstC1.delete();
        }

        //renderVisualization();
    });
})();