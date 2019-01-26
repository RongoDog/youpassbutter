#ifndef GLOBALS_H
#define GLOBALS_H

struct thread_info {
	sem_t *semaphore;
	int *message_queue_id; 
  int *socketfd;
  bool *has_socket_connection;
  bool *client_connected;
};

#endif // GLOBALS_H
