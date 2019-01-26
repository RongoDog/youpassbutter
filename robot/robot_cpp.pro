TEMPLATE = app
TARGET = robot 
INCLUDEPATH += .

SOURCES += ./src/main.cpp \
           ./src/dataflow/communications.cpp \
           ./src/dataflow/socket_connection.cpp \
           ./src/drivers/mpu6050.cpp \
           ./src/drivers/motor_controller.cpp

HEADERS  += ./include/globals.h \
            ./include/dataflow/communications.cpp \
            ./include/dataflow/socket_connection.cpp \           
            ./include/drivers/mpu6050.h \
            ./include/drivers/motor_controller.h \
            ./include/drivers/pinout_definitions.h

INCLUDEPATH += ./include


# The following is for socketio and boost dependencies.

INCLUDEPATH += ./external/sioclient/build/include
DEPENDPATH += ./external/sioclient/build/lib

LIBS += -L./external/sioclient/build/lib/Release/ -lsioclient
LIBS += -L./external/sioclient/build/lib/Release/ -lboost_random
LIBS += -L./external/sioclient/build/lib/Release/ -lboost_system
LIBS += -L./external/sioclient/build/lib/Release/ -lboost_date_time

INCLUDEPATH += /usr/local/include/boost
LIBS += -L/usr/local/lib

INCLUDEPATH += ./external/websocketpp

CONFIG+=no_keywords
CONFIG+=c++11
