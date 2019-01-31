#ifndef GLOBALS_H
#define GLOBALS_H
#include <semaphore.h>
struct thread_info {
  sem_t *semaphore;
  int socketfd;
  bool has_socket_connection;
  bool client_connected;
  unsigned int duty_cycle;
};

#endif // GLOBALS_H
