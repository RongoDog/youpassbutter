#include "communications.h"
#include "chassis.h"
#include <chrono>
#include <thread>

using namespace std::this_thread; // sleep_for, sleep_until
using namespace std::chrono; // nanoseconds, system_clock, seconds

int main()
{
    Chassis *chassis = new Chassis();
    CommunicationsModule *coms = new CommunicationsModule(chassis);
    while(1) {
        sleep_until(system_clock::now() + seconds(1));
    }
}
