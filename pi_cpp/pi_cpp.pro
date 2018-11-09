TEMPLATE = app
TARGET = pi_devastator
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

# The following define makes your compiler warn you if you use any
# feature of Qt which has been marked as deprecated (the exact warnings
# depend on your compiler). Please consult the documentation of the
# deprecated API in order to know how to port your code away from it.
DEFINES += QT_DEPRECATED_WARNINGS

# You can also make your code fail to compile if you use deprecated APIs.
# In order to do so, uncomment the following line.
# You can also select to disable deprecated APIs only up to a certain version of Qt.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0
