function processIncomingData(data) {
  var raw =  window.atob(data);
  var HEX = '';

  for ( i = 0; i < raw.length; i++ ) {

    var _hex = raw.charCodeAt(i).toString(16)

    HEX += (_hex.length==2?_hex:'0'+_hex);

  }
  console.log(HEX.toUpperCase());
}