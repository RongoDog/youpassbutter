#include <sys/types.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "globals.h"
#include "drivers/motor_controller.h"
#include <errno.h>
#define GO_FORWARD 0x01
#define GO_BACKWARD 0x02
#define SHARP_LEFT 0x03
#define SHARP_RIGHT 0x04
#define STOP 0x05
#define TURN_0FF 0x06
char message[1]; 

extern "C" void* initialize_socket_connection(void *args) {
  struct thread_info *info = (struct thread_info *)args;
  int fd = 0, connfd = 0, returned_len = 0;

  fd = socket(AF_UNIX, SOCK_SEQPACKET, 0);
  if (fd < 0) {
    fprintf(stderr, "Failed to create socket file descriptor\n");
    exit(1);
  }
  
  struct sockaddr_un addr;
  addr.sun_family = AF_UNIX;
  strncpy(addr.sun_path, "/tmp/uv4l.socket", sizeof(addr.sun_path)-1);
  
  bind(fd, (struct sockaddr *) &addr, sizeof(addr));
  if (listen(fd, 10)) {
    fprintf(stderr, "Failed to listen on UNIX socket %d\n", errno);
    exit(1);
  }
  socklen_t socket_length = sizeof(addr);
  connfd = accept(fd,(struct sockaddr *)&addr, &socket_length);
  if (connfd < 0) {
    fprintf(stderr, "Failed to accept socket connection\n");
    exit(1);
  }
  char onebytemessage[1]; 
  onebytemessage[0] = 0x77;
  ssize_t sent = send(info->socketfd, onebytemessage, 1, MSG_EOR);
  info->socketfd = connfd;
  info->has_socket_connection = true;

  while (1) {
    returned_len = recv(connfd, message, 1, MGS_WAITALL);
    if (returned_len < 0) {
      fprintf(stderr, "Failed to receive message from socket connection\n");
      continue;
    }
    fprintf(stdout, "Received message %x", message[0]);
    switch(message[0]) {
      case GO_FORWARD:
        drive_forward();
        break;
      case GO_BACKWARD:
        drive_backward();
        break;
      case SHARP_LEFT:
        sharp_left();
        break;
      case SHARP_RIGHT:
        sharp_right();
        break;
      case STOP:
        motors_off();
        break;
      case TURN_0FF:
        system("shutdown -P now");
        exit(0);
        break;
      default:
        continue;
    }
  }

}
