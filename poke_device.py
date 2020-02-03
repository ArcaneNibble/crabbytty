#!/usr/bin/env python3

import usb.core
import usb.util

dev = usb.core.find(idVendor=0xf055, idProduct=0x0000)
assert dev is not None
# print(dev)

# Go into JTAG mode
dev.ctrl_transfer(0x40, 1, 0, 0, None)

def jtag_bit(dev, tms, tdi):
    val = 0
    if tdi:
        val |= 0b01
    if tms:
        val |= 0b10
    tdo = dev.ctrl_transfer(0xC0, 3, val, 0, 1)[0]
    print("tms {} tdi {} tdo {}".format(tms, tdi, tdo))
    return tdo

def go_tlr(dev):
    print("go tlr")
    jtag_bit(dev, 1, 0)
    jtag_bit(dev, 1, 0)
    jtag_bit(dev, 1, 0)
    jtag_bit(dev, 1, 0)
    jtag_bit(dev, 1, 0)

def shift_ir_from_tlr(dev):
    print("shift ir")
    jtag_bit(dev, 0, 0)
    jtag_bit(dev, 1, 0)
    jtag_bit(dev, 1, 0)
    jtag_bit(dev, 0, 0)
    jtag_bit(dev, 0, 0)

def shift_dr_from_tlr(dev):
    print("shift dr")
    jtag_bit(dev, 0, 0)
    jtag_bit(dev, 1, 0)
    jtag_bit(dev, 0, 0)
    bit0 = jtag_bit(dev, 0, 0)
    return bit0

go_tlr(dev)
print("idcode")
idcode = shift_dr_from_tlr(dev)
for i in range(31):
    idcode |= (jtag_bit(dev, 0, 0) << (i + 1))
print("idcode is 0x{:08X}".format(idcode))
# shift_ir_from_tlr(dev)

# print("sniff irlen")
# for _ in range(100):
#     jtag_bit(dev, 0, 1)

# irlen = 0
# for i in range(100):
#     tdo = jtag_bit(dev, 0, 0)
#     if not tdo:
#         irlen = i
#         break
# print("irlen {}".format(irlen))
