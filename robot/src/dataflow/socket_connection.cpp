#include <sys/types.h>
#include <sys/socket.h>
#include <stdio.h>
#include <stdlib.h>

#include "globals.h"
#include "drivers/motor_controller.h"

char message[1]; 

int initialize_socket_connection(void *args) {
  int fd = 0, connfd = 0, returned_len = 0;

  fd = socket(AF_UNIX, SOCK_SEQPACKET, 0);
  if (fd < 0) {
    fprintf("Failed to create socket file descriptor");
    exit(1);
  }

  if (listen(fd, 10)) {
    fprintf("Failed to listen on UNIX socket");
    exit(1);
  }

  connfd = accept(fd, '/tmp/uv4l.socket', 0);
  if (connfd < 0) {
    fprintf("Failed to accept socket connection");
    exit(1);
  }
  args->socketfd = connfd;
  args->has_socket_connection = true;

  while (1) {
    returned_len = recv(connfd, message, 1, 0);
    if (returned_len < 0) {
      fprintf("Failed to receive message from socket connection");
      continue;
    }
    switch(message[0]) {
      case GO_FORWARD:
        drive_forward();
        break;
      case GO_BACKWARD:
        drive_backward();
        break;
      case GO_LEFT:
        sharp_left();
        break;
      case GO_RIGHT:
        sharp_right();
        break;
      case STOP:
        motors_off();
        break;
      default:
        continue;
    }
  }

}