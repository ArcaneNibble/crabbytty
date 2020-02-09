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

_last_bit = 0

def go_tlr(dev):
    print("go tlr")
    jtag_bit(dev, 1, 0)
    jtag_bit(dev, 1, 0)
    jtag_bit(dev, 1, 0)
    jtag_bit(dev, 1, 0)
    jtag_bit(dev, 1, 0)

def rti_from_tlr(dev):
    print("tlr -> rti")
    jtag_bit(dev, 0, 0)

def shift_dr_from_rti(dev):
    global _last_bit
    print("shift dr")
    jtag_bit(dev, 1, 0)
    jtag_bit(dev, 0, 0)
    _last_bit = jtag_bit(dev, 0, 0)

def shift_bits(dev, bits_in):
    global _last_bit
    bits_out = []
    print("shifting {} bits".format(len(bits_in)))
    for i in range(len(bits_in)):
        bits_out.append(_last_bit)
        _last_bit = jtag_bit(dev, 0, bits_in[i])
    return bits_out

def arr2num(arr):
    ret = 0
    for i in range(len(arr)):
        ret |= arr[i] << i
    return ret

def num2arr(num, bits):
    ret = []
    for i in range(bits):
        bit = num & (1 << i)
        ret.append(1 if bit else 0)
    return ret

# def shift_ir_from_tlr(dev):
#     print("shift ir")
#     jtag_bit(dev, 0, 0)
#     jtag_bit(dev, 1, 0)
#     jtag_bit(dev, 1, 0)
#     jtag_bit(dev, 0, 0)
#     jtag_bit(dev, 0, 0)

# def shift_dr_from_tlr(dev):
#     print("shift dr")
#     jtag_bit(dev, 0, 0)
#     jtag_bit(dev, 1, 0)
#     jtag_bit(dev, 0, 0)
#     bit0 = jtag_bit(dev, 0, 0)
#     return bit0

go_tlr(dev)
rti_from_tlr(dev)
shift_dr_from_rti(dev)
print("idcode")
idcode = shift_bits(dev, num2arr(0x12345678, 32))
# idcode = shift_dr_from_tlr(dev)
# for i in range(31):
#     idcode |= (jtag_bit(dev, 0, 0) << (i + 1))
print("idcode is 0x{:08X}".format(arr2num(idcode)))

test = shift_bits(dev, num2arr(0xA53CFACE, 32))
print("test is 0x{:08X}".format(arr2num(test)))
test = shift_bits(dev, [0, 0] * 16)
print("test is 0x{:08X}".format(arr2num(test)))
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
