#!/usr/bin/env python3

import usb.core
import usb.util

dev = usb.core.find(idVendor=0xf055, idProduct=0x0000)
assert dev is not None
# print(dev)
