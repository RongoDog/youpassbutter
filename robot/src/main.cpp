#include "communications.h"
#include "mpu6050.h"
#include "dataflow/socket_connection.h"
#include "drivers/motor_controller.h"
#include <semaphore.h>
#include <stdio.h>
#include <stdlib.h>

extern "C" {
  #include <pigpio.h>
}

int main() {
  // Begin by initializing gpio
  if (gpioInitialise() < 0) {
    fprintf(stderr, "Failed to initialize GPIO interface");
    exit(1);
  }
    
  // The motors should be initially off, but their gpio set
  initialize_motors();
  
  // We need threads for all our sensors and comm modules
  pthread_t accelerometer_thread;
  //pthread_t ultra_sonic_sensor_thread;
  pthread_t socket_connection_thread;  

  // Initialize the semaphore for the i2c communication.
  sem_t i2c_semaphore;
  sem_init(&i2c_semaphore, 0, 1);  

  // Initialize the socket
  int socketfd = 0;
  bool client_connected = false;
  bool has_socket_connection = false;

  // We create the thread info structure
  struct thread_info *info = malloc(sizeof(struct thread_info));
  info->semaphore = &i2c_semaphore;
  info->message_queue_id = &message_queue_id;
  info->socketfd = &socketfd;
  info->client_connected = &client_connected;
  info->has_socket_connection = &has_socket_connection;

  pthread_create(&accelerometer_thread, NULL, initialize_mpu6050, (void *)info);
  pthread_create(&socket_connection_thread, NULL, initialize_socket_connection, (void *)info);
  
  CommunicationsModule *coms = new CommunicationsModule(chassis);
}