import server

# from hanging_threads import start_monitoring
# start_monitoring(seconds_frozen=10, test_interval=100)

# import faulthandler
# faulthandler.enable()

srv = server.PyServer("")

print("srv.init() ", srv.init())

# input()

print("srv.start() ", srv.start())

# input()
# print(srv)

input()

srv.stop()
# input()

import sys;

sys.exit(0)