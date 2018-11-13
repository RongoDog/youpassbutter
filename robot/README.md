# Instructions on how to build the Devastator C++ program

1. Build boost.
    - (from /robot/external/boost)
    - `./bootstrap.sh` [Different on windows]
    - `./b2`
2. Make sure you clone the sioclient code into the correct directory as specified by the .gitmodule file. (use --recurse-submodules)
    - (from /robot) 
    - `git clone --recurse-submodules https://github.com/socketio/socket.io-client-cpp.git ./external/sioclient`
3. We need to update websocketpp to the latest version
    - (from /robot/external/sioclient/lib/websocketpp)
    - `git pull origin master`
4. Now we can build the sioclient libraries
    - (from /robot/external/sioclient)
    - `cmake -DBOOST_ROOT:STRING="../boost" -DBOOST_VER:STRING="1.68.0" ./`
    - `make install`
5. We can now build the C++ program
    - (from /robot)
    - `qmake`
    - `make`
6. It's ready to be run