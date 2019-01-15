CXX_MAC = clang++
CCOPTS_MAC = -c -g -stdlib=libc++ -std=c++11
LINKOPTS_MAC = -g -stdlib=libc++ \
		-lsioclient \
		-lboost_random \
		-lboost_system \
		-lboost_date_time \
		-L./external/sioclient/build/lib/Release/ \
		-pthread
EXEC_MAC=mac_robot

CXX = g++
CCOPTS = -c -g -stdlib=libc++ -std=c++11
LINKOPTS = -g \
		-lrt \
		-lpigpio \
		-lsioclient \
		-lboost_random \
		-lboost_system \
		-lboost_date_time \
		-L./external/sioclient/build/lib/Release/ \
		-pthread
EXEC=robot

INCPATH = -Iinclude -I./external/sioclient/build/include \
		-Iexternal/boost \
		-Iexternal/websocketpp/websocketpp

OBJECTS_MAC= mac_main.o \
  mac_chassis.o \
  mac_communications.o \
	mac_motor_controller.o
OBJECTS= main.o \
  chassis.o \
  communications.o \
	motor_controller.o

all: $(EXEC)

$(EXEC): $(OBJECTS)
	$(CXX): $(LINKOPTS) -o $@ $^

main.o: src/main.cpp \
		include/communications.h \
		include/chassis.h \
		include/drivers/motor_controller.h
	$(CXX) $(CCOPTS) $(INCPATH) -o main.o src/main.cpp

communications.o: src/communications.cpp \
		include/communications.h \
		include/chassis.h \
		include/drivers/motor_controller.h
	$(CXX) $(CCOPTS) $(INCPATH) -o communications.o src/communications.cpp

chassis.o: src/chassis.cpp include/chassis.h \
		include/drivers/motor_controller.h
	$(CXX) $(CCOPTS) $(INCPATH) -o chassis.o src/chassis.cpp

motor_controller.o: src/drivers/motor_controller.cpp include/drivers/pinout_definitions.h \
		include/drivers/motor_controller.h
	$(CXX) $(CCOPTS) $(INCPATH) -o motor_controller.o src/drivers/motor_controller.cpp

mac: $(EXEC_MAC)

$(EXEC_MAC): $(OBJECTS_MAC)
	$(CXX_MAC) $(LINKOPTS_MAC) -o $@ $^

mac_main.o: src/main.cpp \
		include/communications.h \
		include/chassis.h \
		include/drivers/motor_controller.h
	$(CXX_MAC) $(CCOPTS_MAC) $(INCPATH) -o mac_main.o src/main.cpp

mac_communications.o: src/communications.cpp \
		include/communications.h \
		include/chassis.h \
		include/drivers/motor_controller.h
	$(CXX_MAC) $(CCOPTS_MAC) $(INCPATH) -o mac_communications.o src/communications.cpp

mac_chassis.o: src/chassis.cpp include/chassis.h \
		include/drivers/motor_controller.h
	$(CXX_MAC) $(CCOPTS_MAC) $(INCPATH) -o mac_chassis.o src/chassis.cpp

mac_motor_controller.o: src/drivers/motor_controller.cpp include/drivers/pinout_definitions.h \
		include/drivers/motor_controller.h
	$(CXX_MAC) $(CCOPTS_MAC) $(INCPATH) -o mac_motor_controller.o src/drivers/motor_controller.cpp

clean:
	- $(RM) $(EXEC_MAC) $(EXEC)
	- $(RM) $(OBJECTS_MAC) $(OBJECTS)
	- $(RM) *~
	- $(RM) core.*

pretty: 
	indent *.c *.h -kr
