#include "drivers/pinout_definitions.h"
#include "globals.h"
#include "utils/base64.h"

#include <stdio.h>
#include <stdlib.h>
#include <semaphore.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <vector>

extern "C" {
  #include <pigpio.h>
}

// We reset the device to it's initial conditions
#define PWR_MGMT_1								0x6B
#define RESET_PWR									0x80

// sample rate divider to set freq at which accel data is read
// DLPF is enabled, so the gyroscope frequency is 8 KHz.
// If we want a sample rate of 100 Hz, the SMPLRT_DIV value should be 1000/100 - 1 = 4
#define SAMPLE_RATE_REG           0x19
#define SMPRT_DIV                 0x09

// We are not using FSYNC and we want to reduce high frequencey signals
// at the cost of delay. (Bandwidth = 44Hz, Delay = 8.5ms)
#define CONFIG_REG                0x1A
#define DLPG_CONFIG               0x03

// Gyroscope sensitivity set to +- 250 degrees/s
#define GYRO_CONFIG_REG           0x1B
#define GYRO_X_CONFIG             0x80
#define GYRO_Y_CONFIG             0x40
#define GYRO_Z_CONFIG             0x20

// Accelerometer sensitivty set to +- 4g for each direction
#define ACCEL_CONFIG_REG          0x1C
#define ACCEL_X_CONFIG            0x88
#define ACCEL_Y_CONFIG            0x48
#define ACCEL_Z_CONFIG            0x28

// We enable the FIFO for accelerometer and gyroscope values
#define FIFO_EN_REG               0x23
#define FIFO_EN_CONFIG            0x78

// We need to clear the FIFO and enable it.
#define USER_CTRL_REG             0x6A
#define FIFO_RESET                0x05
#define FIFO_EN                   0x40

// Contents of the FIFO buffer
#define FIFO_R_W_REG              0x74   // R/W

// Amount of data stored in the FIFO read H->L
#define FIFO_COUNT_H_REG          0x72 //   //R
#define FIFO_COUNT_L_REG          0x73 //   //R 


extern "C" void* initialize_mpu6050(void *arg){
	struct thread_info *info = (struct thread_info *)arg;
	sem_t *i2c_semaphore = info->semaphore;
	// We block until WebRTC is available and the client is connected
	while (!(info->has_socket_connection) | !(info->client_connected)) {
		gpioDelay(1*MICRO_SEC_IN_SEC);
	}
	sem_wait(i2c_semaphore);
	int handle;
	handle = i2cOpen(1, MPU6050_ADDRESS, 0);
  if (handle < 0) {
		if (handle == PI_BAD_I2C_BUS) {
			fprintf(stderr, "Bad I2C Bus\n");
		} else if (handle == PI_BAD_I2C_ADDR) {
			fprintf(stderr, "Bad address\n");
		} else if (handle == PI_NO_HANDLE) {
			fprintf(stderr, "No handle\n");
		} else if (handle == PI_I2C_OPEN_FAILED) {
			fprintf(stderr, "Open failed\n");
		} else {
			fprintf(stderr, "Failed to open i2c bus, %d\n", handle);
		}
		exit(1);
	}
	int response = i2cWriteByteData(handle, PWR_MGMT_1, 0x00); 
	if (response != 0) {
		if (response == PI_BAD_HANDLE) {
			fprintf(stderr, "Bad handle for MPU6050 PWR register\n");
		} else if (response == PI_BAD_PARAM) {
			fprintf(stderr, "Bad parameters for MPU6050 PWR register\n");
		} else if (response == PI_I2C_WRITE_FAILED) {
			fprintf(stderr, "Write file for MPU6050 PWR register\n");
		}
		exit(1);
	}
	response = i2cWriteByteData(handle, CONFIG_REG, DLPG_CONFIG);
	if (response != 0) {
		if (response == PI_BAD_HANDLE) {
			fprintf(stderr, "Bad handle for MPU6050 config register\n");
		} else if (response == PI_BAD_PARAM) {
			fprintf(stderr, "Bad parameters for MPU6050 config register\n");
		} else if (response == PI_I2C_WRITE_FAILED) {
			fprintf(stderr, "Write file for MPU6050 config register\n");
		}
		exit(1);
	}
	

  response = i2cWriteByteData(handle, SAMPLE_RATE_REG, SMPRT_DIV);
	if (response != 0) {
		if (response == PI_BAD_HANDLE) {
			fprintf(stderr, "Bad handle for MPU6050 sample rate register\n");
		} else if (response == PI_BAD_PARAM) {
			fprintf(stderr, "Bad parameters for MPU6050 sample rate register\n");
		} else if (response == PI_I2C_WRITE_FAILED) {
			fprintf(stderr, "Write file for MPU6050 sample rate register\n");
		}
		exit(1);
	}

	int gyro_configs[3] = { GYRO_X_CONFIG, GYRO_Y_CONFIG, GYRO_Z_CONFIG };
	int i;
	for (i = 0; i < 3; i ++) {
  	response = i2cWriteByteData(handle, GYRO_CONFIG_REG, gyro_configs[i]);
		if (response != 0) {
			if (response == PI_BAD_HANDLE) {
				fprintf(stderr, "Bad handle for MPU6050 gyroscope configuration register\n");
			} else if (response == PI_BAD_PARAM) {
				fprintf(stderr, "Bad parameters for MPU6050 gyroscope configuration register\n");
			} else if (response == PI_I2C_WRITE_FAILED) {
				fprintf(stderr, "Write file for MPU6050 gyroscope configuration register\n");
			}
			exit(1);
		}
	}

	int accel_configs[3] = { ACCEL_X_CONFIG, ACCEL_Y_CONFIG, ACCEL_Z_CONFIG };
	for (i = 0; i < 3; i ++) {
  	response = i2cWriteByteData(handle, ACCEL_CONFIG_REG, accel_configs[i]);
		if (response != 0) {
			if (response == PI_BAD_HANDLE) {
				fprintf(stderr, "Bad handle for MPU6050 accelerometer configuration register\n");
			} else if (response == PI_BAD_PARAM) {
				fprintf(stderr, "Bad parameters for MPU6050 accelerometer configuration register\n");
			} else if (response == PI_I2C_WRITE_FAILED) {
				fprintf(stderr, "Write file for MPU6050 accelerometer configuration register\n");
			}
			exit(1);
		}
	}

	response = i2cWriteByteData(handle, FIFO_EN_REG, FIFO_EN_CONFIG);
	if (response != 0) {
		if (response == PI_BAD_HANDLE) {
			fprintf(stderr, "Bad handle for MPU6050 FIFO enable register\n");
		} else if (response == PI_BAD_PARAM) {
			fprintf(stderr, "Bad parameters for MPU6050 FIFO enable register\n");
		} else if (response == PI_I2C_WRITE_FAILED) {
			fprintf(stderr, "Write file for MPU6050 FIFO enable register\n");
		}
		exit(1);
	}

	response = i2cWriteByteData(handle, USER_CTRL_REG, FIFO_RESET);
	if (response != 0) {
		if (response == PI_BAD_HANDLE) {
			fprintf(stderr, "Bad handle for MPU6050 FIFO reset command\n");
		} else if (response == PI_BAD_PARAM) {
			fprintf(stderr, "Bad parameters for MPU6050 FIFO reset command\n");
		} else if (response == PI_I2C_WRITE_FAILED) {
			fprintf(stderr, "Write file for MPU6050 FIFO reset command\n");
		}
		exit(1);
	}

	response = i2cWriteByteData(handle, USER_CTRL_REG, FIFO_EN);
	if (response != 0) {
		if (response == PI_BAD_HANDLE) {
			fprintf(stderr, "Bad handle for MPU6050 FIFO enable command\n");
		} else if (response == PI_BAD_PARAM) {
			fprintf(stderr, "Bad parameters for MPU6050 FIFO enable command\n");
		} else if (response == PI_I2C_WRITE_FAILED) {
			fprintf(stderr, "Write file for MPU6050 FIFO enable command\n");
		}
		exit(1);
	}
	sem_post(i2c_semaphore);
	// Main while loop for reading data from FIFO
	while(1) {
		// We sample in chunks 5 times a second
	  gpioDelay(0.10*MICRO_SEC_IN_SEC);

		// We read the high and low bits of the FIFO count
		sem_wait(i2c_semaphore);
		int high_count_byte = i2cReadByteData(handle, FIFO_COUNT_H_REG);
		if (high_count_byte < 0) {
			fprintf(stderr, "Failed to read MPU6050 high FIFO count word\n");
			sem_post(i2c_semaphore);
			continue;
		}
		int low_count_byte = i2cReadByteData(handle, FIFO_COUNT_L_REG);
		if (low_count_byte < 0) {
			fprintf(stderr, "Failed to read MPU6050 low FIFO count word\n");
			sem_post(i2c_semaphore);
			continue;
		}
		int count_value = (high_count_byte<<8) + low_count_byte;
		// We intend on reading a full dataset at a time. For 12 bytes. As such we perform a modulus operation
		// to determine any extra bytes. 
		int nb_reads = count_value - (count_value % 12);
		char buffer[nb_reads]; 
		int total_read = 0;
		while (total_read < nb_reads) {
			int to_read = 32;
			if ((nb_reads - total_read) < 32) {
				to_read = (nb_reads-total_read);
			}
			response = i2cReadI2CBlockData(handle, FIFO_R_W_REG, (char *)buffer + total_read, to_read);
			if (response < 0) {
				fprintf(stderr, "Failed to read MPU6050 FIFO");
				break;
			}
			total_read += to_read;
		}

		// If we've read some data, we send it over the websocket we assume is open
		if (total_read > 0) {
			size_t encoded_length; 
			unsigned char *encoded = base64_encode((const unsigned char*)buffer, total_read, &encoded_length);
			fprintf(stdout, "The output buffer %s\n", encoded);
			ssize_t sent = send(info->socketfd, encoded, encoded_length, MSG_EOR);
			if (sent < 0) {
				fprintf(stderr, "Failed to send all necessary MPU6050 data");
			}
			free(encoded);
		}
		sem_post(i2c_semaphore);
	}
}
