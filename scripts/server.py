import time
import con_server
import con_common

import py_server_emulator

from typing import Optional
import threading

class PyServer:
    def __init__(self, plugin: str):
        self.con_servers = dict()
        self.con_commons = dict()

        self.mt5_srv = py_server_emulator.Server.new(self, plugin)
        self.storage = self.mt5_srv.storage()

    def init(self):
        self.mt5_srv.init()

    def start(self):
        self.mt5_srv.start()

    def stop(self):
        self.mt5_srv.stop()

    def about(self, info):
        return 0

    def license_check(self, license_name):
        return 0

    def common_create(self) -> Optional[int]:
        common_impl = con_common.ConCommon()
        common_id = self.storage.alloc_con_common(common_impl)
        self.con_commons[common_id] = common_impl

        return common_id

    def common_get(self, con_common) -> int:
        return 0

    def time_current_msc(self) -> int:
        print("time_current_msc() was called")
        return time.time_ns() // 1_000_000

    def net_server_create(self) -> Optional[int]:
        con_srv = con_server.PyConServer()
        con_server_id = self.storage.alloc_con_server(con_srv)
        self.con_servers[con_server_id] = con_srv

        return con_server_id