function setupKeyCommands(dataChannel) {
  var bufferArray = new ArrayBuffer(4);
  var byteArray = new Uint8Array(bufferArray);
  button = document.getElementById("SEND");
  ctrl = document.getElementById("CONTROL");
  button.addEventListener("click", (event) => {
    event.preventDefault();
    byteArray[0] = 3;
    byteArray[1] = Number(ctrl.value);
    dataChannel.send(byteArray);
  });
  document.addEventListener("keypress", (event) => {
    event.preventDefault();
    switch (event.code) {
      case "KeyW":
        byteArray[0] = 2;
        byteArray[1] = 1;
        dataChannel.send(byteArray);
        break;
      case "KeyA":
        byteArray[0] = 2;
        byteArray[1] = 3;
        dataChannel.send(byteArray);
        break;
      case "KeyD":
        byteArray[0] = 2;
        byteArray[1] = 4;
        dataChannel.send(byteArray);
        break;
      case "KeyS":
        byteArray[0] = 2;
        byteArray[1] = 2;
        dataChannel.send(byteArray);
        break;
      case "KeyE":
        byteArray[0] = 5;
        dataChannel.send(byteArray);
        break;
      case "KeyO":
        byteArray[0] = 6;
        dataChannel.send(byteArray);
        break;
      default:
        break;
    }
  });
}
