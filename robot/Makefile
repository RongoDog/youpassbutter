#############################################################################
# Makefile for building: robot
# Generated by qmake (3.0) (Qt 5.7.1)
# Project:  robot_cpp.pro
# Template: app
# Command: /usr/lib/arm-linux-gnueabihf/qt5/bin/qmake -o Makefile robot_cpp.pro
#############################################################################

MAKEFILE      = Makefile

####### Compiler, tools and options

CC            = gcc
CXX           = g++
DEFINES       = -DQT_NO_DEBUG -DQT_NO_KEYWORDS -DQT_GUI_LIB -DQT_CORE_LIB
CFLAGS        = -pipe -O2 -Wall -W -D_REENTRANT -fPIC $(DEFINES)
CXXFLAGS      = -pipe -O2 -std=gnu++11 -Wall -W -D_REENTRANT -fPIC $(DEFINES)
INCPATH       = -I. -I. -Iinclude -Iexternal/sioclient/build/include -isystem /usr/local/include/boost -Iexternal/websocketpp -isystem /usr/include/arm-linux-gnueabihf/qt5 -isystem /usr/include/arm-linux-gnueabihf/qt5/QtGui -isystem /usr/include/arm-linux-gnueabihf/qt5/QtCore -I. -I/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/linux-g++
QMAKE         = /usr/lib/arm-linux-gnueabihf/qt5/bin/qmake
DEL_FILE      = rm -f
CHK_DIR_EXISTS= test -d
MKDIR         = mkdir -p
COPY          = cp -f
COPY_FILE     = cp -f
COPY_DIR      = cp -f -R
INSTALL_FILE  = install -m 644 -p
INSTALL_PROGRAM = install -m 755 -p
INSTALL_DIR   = cp -f -R
DEL_FILE      = rm -f
SYMLINK       = ln -f -s
DEL_DIR       = rmdir
MOVE          = mv -f
TAR           = tar -cf
COMPRESS      = gzip -9f
DISTNAME      = robot1.0.0
DISTDIR = /home/pi/youpassbutter/robot/.tmp/robot1.0.0
LINK          = g++
LFLAGS        = -Wl,-O1
LIBS          = $(SUBLIBS) -L./external/sioclient/build/lib/Release/ -lsioclient -lboost_random -lboost_system -lboost_date_time -L/usr/local/lib -lpigpio -lQt5Gui -lQt5Core -lGLESv2 -lpthread 
AR            = ar cqs
RANLIB        = 
SED           = sed
STRIP         = strip

####### Output directory

OBJECTS_DIR   = ./

####### Files

SOURCES       = src/main.cpp \
		src/dataflow/communications.cpp \
		src/dataflow/socket_connection.cpp \
		src/drivers/mpu6050.cpp \
		src/drivers/motor_controller.cpp 
OBJECTS       = main.o \
		communications.o \
		socket_connection.o \
		mpu6050.o \
		motor_controller.o
DIST          = /usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/spec_pre.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/unix.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/linux.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/sanitize.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/gcc-base.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/gcc-base-unix.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/g++-base.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/g++-unix.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/qconfig.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_bootstrap_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_concurrent.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_concurrent_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_core.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_core_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_dbus.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_dbus_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_eglfs_device_lib_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_eglfs_kms_support_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_gui.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_gui_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_network.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_network_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_opengl.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_opengl_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_openglextensions.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_openglextensions_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_platformsupport_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_printsupport.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_printsupport_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_sql.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_sql_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_testlib.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_testlib_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_widgets.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_widgets_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_xcb_qpa_lib_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_xml.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_xml_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/qt_functions.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/qt_config.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/linux-g++/qmake.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/spec_post.prf \
		.qmake.stash \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/exclusive_builds.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/toolchain.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/default_pre.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/resolve_config.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/default_post.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/warn_on.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/qt.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/resources.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/moc.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/unix/opengl.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/unix/thread.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/file_copies.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/testcase_targets.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/exceptions.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/yacc.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/lex.prf \
		robot_cpp.pro include/globals.h \
		include/dataflow/communications.h \
		include/dataflow/socket_connection.h \
		include/drivers/mpu6050.h \
		include/drivers/motor_controller.h \
		include/drivers/pinout_definitions.h src/main.cpp \
		src/dataflow/communications.cpp \
		src/dataflow/socket_connection.cpp \
		src/drivers/mpu6050.cpp \
		src/drivers/motor_controller.cpp
QMAKE_TARGET  = robot
DESTDIR       = 
TARGET        = robot


first: all
####### Build rules

$(TARGET):  $(OBJECTS)  
	$(LINK) $(LFLAGS) -o $(TARGET) $(OBJECTS) $(OBJCOMP) $(LIBS)

Makefile: robot_cpp.pro /usr/lib/arm-linux-gnueabihf/qt5/mkspecs/linux-g++/qmake.conf /usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/spec_pre.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/unix.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/linux.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/sanitize.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/gcc-base.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/gcc-base-unix.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/g++-base.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/g++-unix.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/qconfig.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_bootstrap_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_concurrent.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_concurrent_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_core.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_core_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_dbus.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_dbus_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_eglfs_device_lib_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_eglfs_kms_support_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_gui.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_gui_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_network.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_network_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_opengl.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_opengl_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_openglextensions.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_openglextensions_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_platformsupport_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_printsupport.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_printsupport_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_sql.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_sql_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_testlib.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_testlib_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_widgets.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_widgets_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_xcb_qpa_lib_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_xml.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_xml_private.pri \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/qt_functions.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/qt_config.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/linux-g++/qmake.conf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/spec_post.prf \
		.qmake.stash \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/exclusive_builds.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/toolchain.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/default_pre.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/resolve_config.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/default_post.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/warn_on.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/qt.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/resources.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/moc.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/unix/opengl.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/unix/thread.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/file_copies.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/testcase_targets.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/exceptions.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/yacc.prf \
		/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/lex.prf \
		robot_cpp.pro \
		/usr/lib/arm-linux-gnueabihf/libQt5Gui.prl \
		/usr/lib/arm-linux-gnueabihf/libQt5Core.prl
	$(QMAKE) -o Makefile robot_cpp.pro
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/spec_pre.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/unix.conf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/linux.conf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/sanitize.conf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/gcc-base.conf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/gcc-base-unix.conf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/g++-base.conf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/common/g++-unix.conf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/qconfig.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_bootstrap_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_concurrent.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_concurrent_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_core.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_core_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_dbus.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_dbus_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_eglfs_device_lib_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_eglfs_kms_support_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_gui.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_gui_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_network.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_network_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_opengl.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_opengl_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_openglextensions.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_openglextensions_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_platformsupport_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_printsupport.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_printsupport_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_sql.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_sql_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_testlib.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_testlib_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_widgets.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_widgets_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_xcb_qpa_lib_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_xml.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/modules/qt_lib_xml_private.pri:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/qt_functions.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/qt_config.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/linux-g++/qmake.conf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/spec_post.prf:
.qmake.stash:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/exclusive_builds.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/toolchain.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/default_pre.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/resolve_config.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/default_post.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/warn_on.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/qt.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/resources.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/moc.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/unix/opengl.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/unix/thread.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/file_copies.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/testcase_targets.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/exceptions.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/yacc.prf:
/usr/lib/arm-linux-gnueabihf/qt5/mkspecs/features/lex.prf:
robot_cpp.pro:
/usr/lib/arm-linux-gnueabihf/libQt5Gui.prl:
/usr/lib/arm-linux-gnueabihf/libQt5Core.prl:
qmake: FORCE
	@$(QMAKE) -o Makefile robot_cpp.pro

qmake_all: FORCE


all: Makefile $(TARGET)

dist: distdir FORCE
	(cd `dirname $(DISTDIR)` && $(TAR) $(DISTNAME).tar $(DISTNAME) && $(COMPRESS) $(DISTNAME).tar) && $(MOVE) `dirname $(DISTDIR)`/$(DISTNAME).tar.gz . && $(DEL_FILE) -r $(DISTDIR)

distdir: FORCE
	@test -d $(DISTDIR) || mkdir -p $(DISTDIR)
	$(COPY_FILE) --parents $(DIST) $(DISTDIR)/
	$(COPY_FILE) --parents include/globals.h include/dataflow/communications.h include/dataflow/socket_connection.h include/drivers/mpu6050.h include/drivers/motor_controller.h include/drivers/pinout_definitions.h $(DISTDIR)/
	$(COPY_FILE) --parents src/main.cpp src/dataflow/communications.cpp src/dataflow/socket_connection.cpp src/drivers/mpu6050.cpp src/drivers/motor_controller.cpp $(DISTDIR)/


clean: compiler_clean 
	-$(DEL_FILE) $(OBJECTS)
	-$(DEL_FILE) *~ core *.core


distclean: clean 
	-$(DEL_FILE) $(TARGET) 
	-$(DEL_FILE) .qmake.stash
	-$(DEL_FILE) Makefile


####### Sub-libraries

mocclean: compiler_moc_header_clean compiler_moc_source_clean

mocables: compiler_moc_header_make_all compiler_moc_source_make_all

check: first

benchmark: first

compiler_rcc_make_all:
compiler_rcc_clean:
compiler_moc_header_make_all:
compiler_moc_header_clean:
compiler_moc_source_make_all:
compiler_moc_source_clean:
compiler_yacc_decl_make_all:
compiler_yacc_decl_clean:
compiler_yacc_impl_make_all:
compiler_yacc_impl_clean:
compiler_lex_make_all:
compiler_lex_clean:
compiler_clean: 

####### Compile

main.o: src/main.cpp include/dataflow/communications.h \
		external/websocketpp/websocketpp/client.hpp \
		external/websocketpp/websocketpp/roles/client_endpoint.hpp \
		external/websocketpp/websocketpp/endpoint.hpp \
		external/websocketpp/websocketpp/connection.hpp \
		external/websocketpp/websocketpp/close.hpp \
		external/websocketpp/websocketpp/error.hpp \
		external/websocketpp/websocketpp/common/cpp11.hpp \
		external/websocketpp/websocketpp/common/system_error.hpp \
		external/websocketpp/websocketpp/common/network.hpp \
		external/websocketpp/websocketpp/common/stdint.hpp \
		external/websocketpp/websocketpp/utf8_validator.hpp \
		external/websocketpp/websocketpp/frame.hpp \
		external/websocketpp/websocketpp/utilities.hpp \
		external/websocketpp/websocketpp/impl/utilities_impl.hpp \
		external/websocketpp/websocketpp/logger/levels.hpp \
		external/websocketpp/websocketpp/processors/processor.hpp \
		external/websocketpp/websocketpp/processors/base.hpp \
		external/websocketpp/websocketpp/uri.hpp \
		external/websocketpp/websocketpp/common/memory.hpp \
		external/websocketpp/websocketpp/transport/base/connection.hpp \
		external/websocketpp/websocketpp/common/connection_hdl.hpp \
		external/websocketpp/websocketpp/common/functional.hpp \
		external/websocketpp/websocketpp/http/constants.hpp \
		external/websocketpp/websocketpp/impl/connection_impl.hpp \
		external/websocketpp/websocketpp/processors/hybi00.hpp \
		external/websocketpp/websocketpp/common/md5.hpp \
		external/websocketpp/websocketpp/common/platforms.hpp \
		external/websocketpp/websocketpp/processors/hybi07.hpp \
		external/websocketpp/websocketpp/processors/hybi08.hpp \
		external/websocketpp/websocketpp/processors/hybi13.hpp \
		external/websocketpp/websocketpp/sha1/sha1.hpp \
		external/websocketpp/websocketpp/base64/base64.hpp \
		external/websocketpp/websocketpp/version.hpp \
		external/websocketpp/websocketpp/impl/endpoint_impl.hpp \
		external/websocketpp/websocketpp/config/asio_no_tls_client.hpp \
		external/websocketpp/websocketpp/config/core_client.hpp \
		external/websocketpp/websocketpp/concurrency/basic.hpp \
		external/websocketpp/websocketpp/common/thread.hpp \
		external/websocketpp/websocketpp/concurrency/none.hpp \
		external/websocketpp/websocketpp/transport/iostream/endpoint.hpp \
		external/websocketpp/websocketpp/transport/base/endpoint.hpp \
		external/websocketpp/websocketpp/transport/iostream/connection.hpp \
		external/websocketpp/websocketpp/transport/iostream/base.hpp \
		external/websocketpp/websocketpp/http/request.hpp \
		external/websocketpp/websocketpp/http/parser.hpp \
		external/websocketpp/websocketpp/http/impl/parser.hpp \
		external/websocketpp/websocketpp/http/impl/request.hpp \
		external/websocketpp/websocketpp/http/response.hpp \
		external/websocketpp/websocketpp/http/impl/response.hpp \
		external/websocketpp/websocketpp/message_buffer/message.hpp \
		external/websocketpp/websocketpp/message_buffer/alloc.hpp \
		external/websocketpp/websocketpp/logger/basic.hpp \
		external/websocketpp/websocketpp/common/time.hpp \
		external/websocketpp/websocketpp/random/random_device.hpp \
		external/websocketpp/websocketpp/common/random.hpp \
		external/websocketpp/websocketpp/endpoint_base.hpp \
		external/websocketpp/websocketpp/connection_base.hpp \
		external/websocketpp/websocketpp/extensions/permessage_deflate/disabled.hpp \
		external/websocketpp/websocketpp/extensions/extension.hpp \
		external/websocketpp/websocketpp/transport/asio/endpoint.hpp \
		external/websocketpp/websocketpp/transport/asio/connection.hpp \
		external/websocketpp/websocketpp/transport/asio/base.hpp \
		external/websocketpp/websocketpp/common/asio.hpp \
		external/websocketpp/websocketpp/common/chrono.hpp \
		external/websocketpp/websocketpp/common/type_traits.hpp \
		external/websocketpp/websocketpp/transport/asio/security/none.hpp \
		external/websocketpp/websocketpp/transport/asio/security/base.hpp \
		external/sioclient/build/include/sio_client.h \
		external/sioclient/build/include/sio_message.h \
		external/sioclient/build/include/sio_socket.h \
		include/globals.h \
		include/drivers/mpu6050.h \
		include/dataflow/socket_connection.h \
		include/drivers/motor_controller.h
	$(CXX) -c $(CXXFLAGS) $(INCPATH) -o main.o src/main.cpp

communications.o: src/dataflow/communications.cpp include/dataflow/communications.h \
		external/websocketpp/websocketpp/client.hpp \
		external/websocketpp/websocketpp/roles/client_endpoint.hpp \
		external/websocketpp/websocketpp/endpoint.hpp \
		external/websocketpp/websocketpp/connection.hpp \
		external/websocketpp/websocketpp/close.hpp \
		external/websocketpp/websocketpp/error.hpp \
		external/websocketpp/websocketpp/common/cpp11.hpp \
		external/websocketpp/websocketpp/common/system_error.hpp \
		external/websocketpp/websocketpp/common/network.hpp \
		external/websocketpp/websocketpp/common/stdint.hpp \
		external/websocketpp/websocketpp/utf8_validator.hpp \
		external/websocketpp/websocketpp/frame.hpp \
		external/websocketpp/websocketpp/utilities.hpp \
		external/websocketpp/websocketpp/impl/utilities_impl.hpp \
		external/websocketpp/websocketpp/logger/levels.hpp \
		external/websocketpp/websocketpp/processors/processor.hpp \
		external/websocketpp/websocketpp/processors/base.hpp \
		external/websocketpp/websocketpp/uri.hpp \
		external/websocketpp/websocketpp/common/memory.hpp \
		external/websocketpp/websocketpp/transport/base/connection.hpp \
		external/websocketpp/websocketpp/common/connection_hdl.hpp \
		external/websocketpp/websocketpp/common/functional.hpp \
		external/websocketpp/websocketpp/http/constants.hpp \
		external/websocketpp/websocketpp/impl/connection_impl.hpp \
		external/websocketpp/websocketpp/processors/hybi00.hpp \
		external/websocketpp/websocketpp/common/md5.hpp \
		external/websocketpp/websocketpp/common/platforms.hpp \
		external/websocketpp/websocketpp/processors/hybi07.hpp \
		external/websocketpp/websocketpp/processors/hybi08.hpp \
		external/websocketpp/websocketpp/processors/hybi13.hpp \
		external/websocketpp/websocketpp/sha1/sha1.hpp \
		external/websocketpp/websocketpp/base64/base64.hpp \
		external/websocketpp/websocketpp/version.hpp \
		external/websocketpp/websocketpp/impl/endpoint_impl.hpp \
		external/websocketpp/websocketpp/config/asio_no_tls_client.hpp \
		external/websocketpp/websocketpp/config/core_client.hpp \
		external/websocketpp/websocketpp/concurrency/basic.hpp \
		external/websocketpp/websocketpp/common/thread.hpp \
		external/websocketpp/websocketpp/concurrency/none.hpp \
		external/websocketpp/websocketpp/transport/iostream/endpoint.hpp \
		external/websocketpp/websocketpp/transport/base/endpoint.hpp \
		external/websocketpp/websocketpp/transport/iostream/connection.hpp \
		external/websocketpp/websocketpp/transport/iostream/base.hpp \
		external/websocketpp/websocketpp/http/request.hpp \
		external/websocketpp/websocketpp/http/parser.hpp \
		external/websocketpp/websocketpp/http/impl/parser.hpp \
		external/websocketpp/websocketpp/http/impl/request.hpp \
		external/websocketpp/websocketpp/http/response.hpp \
		external/websocketpp/websocketpp/http/impl/response.hpp \
		external/websocketpp/websocketpp/message_buffer/message.hpp \
		external/websocketpp/websocketpp/message_buffer/alloc.hpp \
		external/websocketpp/websocketpp/logger/basic.hpp \
		external/websocketpp/websocketpp/common/time.hpp \
		external/websocketpp/websocketpp/random/random_device.hpp \
		external/websocketpp/websocketpp/common/random.hpp \
		external/websocketpp/websocketpp/endpoint_base.hpp \
		external/websocketpp/websocketpp/connection_base.hpp \
		external/websocketpp/websocketpp/extensions/permessage_deflate/disabled.hpp \
		external/websocketpp/websocketpp/extensions/extension.hpp \
		external/websocketpp/websocketpp/transport/asio/endpoint.hpp \
		external/websocketpp/websocketpp/transport/asio/connection.hpp \
		external/websocketpp/websocketpp/transport/asio/base.hpp \
		external/websocketpp/websocketpp/common/asio.hpp \
		external/websocketpp/websocketpp/common/chrono.hpp \
		external/websocketpp/websocketpp/common/type_traits.hpp \
		external/websocketpp/websocketpp/transport/asio/security/none.hpp \
		external/websocketpp/websocketpp/transport/asio/security/base.hpp \
		external/sioclient/build/include/sio_client.h \
		external/sioclient/build/include/sio_message.h \
		external/sioclient/build/include/sio_socket.h \
		include/globals.h
	$(CXX) -c $(CXXFLAGS) $(INCPATH) -o communications.o src/dataflow/communications.cpp

socket_connection.o: src/dataflow/socket_connection.cpp include/globals.h \
		include/drivers/motor_controller.h
	$(CXX) -c $(CXXFLAGS) $(INCPATH) -o socket_connection.o src/dataflow/socket_connection.cpp

mpu6050.o: src/drivers/mpu6050.cpp include/drivers/pinout_definitions.h \
		include/globals.h
	$(CXX) -c $(CXXFLAGS) $(INCPATH) -o mpu6050.o src/drivers/mpu6050.cpp

motor_controller.o: src/drivers/motor_controller.cpp include/drivers/pinout_definitions.h \
		include/drivers/motor_controller.h
	$(CXX) -c $(CXXFLAGS) $(INCPATH) -o motor_controller.o src/drivers/motor_controller.cpp

####### Install

install:  FORCE

uninstall:  FORCE

FORCE:

