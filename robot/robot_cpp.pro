TEMPLATE = app
TARGET = robot 
INCLUDEPATH += .

SOURCES += ./src/main.cpp \
           ./src/communications.cpp \
           ./src/chassis.cpp \
           ./src/drivers/motor_controller.cpp

HEADERS  += ./include/communications.h \
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

INCLUDEPATH += /usr/local/include/boost
LIBS += -L/usr/local/lib

INCLUDEPATH += ./external/websocketpp

CONFIG+=no_keywords
CONFIG+=c++11
