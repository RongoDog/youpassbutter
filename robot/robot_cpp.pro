TEMPLATE = app
TARGET = robot 
INCLUDEPATH += .

SOURCES += ./src/main.cpp \
           ./src/socket_handler.cpp \
           ./src/chassis.cpp \
           ./src/drivers/motor_controller.cpp

HEADERS  += ./include/socket_handler.h \
            ./include/chassis.h \
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

INCLUDEPATH += ./external/boost
LIBS += -L./external/boost/stage/lib

CONFIG+=no_keywords
CONFIG+=c++11