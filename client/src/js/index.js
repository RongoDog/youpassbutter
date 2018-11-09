// initialize stats (fps display)
var socket = io.connect('http://localhost:3000');

socket.emit("join", "client");

socket.on('bot-command', (data) => {
    console.log(data);
});

// creating a scene with some light and a camera to view it
var scene = new THREE.Scene();
scene.background = new THREE.Color(0x9080ff);
scene.add(new THREE.AmbientLight(0xffffff));
var camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);

// rendering the camera's view on the screen
var renderer = new THREE.WebGLRenderer();
renderer.setPixelRatio(window.devicePixelRatio);
renderer.setSize(window.innerWidth, window.innerHeight);
//renderer.setClearColor(new THREE.Color(0xE1E3EE, 1.0));
document.body.appendChild(renderer.domElement);

// adding the vehicle to the scene, modeled as a cube for now
var geometry = new THREE.BoxGeometry(1, 1, 1);
var material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
var cube = new THREE.Mesh(geometry, material);
scene.add(cube);

camera.position.set(0, 0, 7);
//camera.lookAt(cube.position());

function animate() {
    requestAnimationFrame(animate);

    var xSpeed = 3.0;
    var ySpeed = 3.0;
    renderer.render(scene, camera);
};
animate();

window.addEventListener("load", (e) => {
    document.addEventListener("keypress", (event) => {
        event.preventDefault();
        switch (event.code) {
            case "KeyW":
                socket.emit('bot-command', "forward", (data) => {
                    if (data.length > 0){
                        console.log("Sent Forward Command");
                    }
                });
                break;
            case "KeyA":
                socket.emit('bot-command', "left", (data) => {
                    if (data.length > 0){
                        console.log("Sent Left Command");
                    }
                });
                break;
            case "KeyD":
                socket.emit('bot-command', "right", (data) => {
                    if (data.length > 0){
                        console.log("Sent Right Command");
                    }
                });
                break;
            case "KeyS":
                socket.emit('bot-command', "backward", (data) => {
                    if (data.length > 0){
                        console.log("Sent Backward Command");
                    }
                });
                break;
            default:
                break;
        }
    });
});
