function setupKeyCommands(dataChannel) {
  var bufferArray = new ArrayBuffer(1);
  var byteArray = new Uint8Array(bufferArray);

  document.addEventListener("keypress", (event) => {
    event.preventDefault();
    switch (event.code) {
      case "KeyW":
        byteArray[0] = 1;
        dataChannel.send(byteArray);
        break;
      case "KeyA":
        byteArray[0] = 2;
        dataChannel.send(byteArray);
        break;
      case "KeyD":
        byteArray[0] = 3;
        dataChannel.send(byteArray);
        break;
      case "KeyS":
        byteArray[0] = 4;
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
