#!/bin/bash

modprobe zram num_devices=2

echo 16G > /sys/block/zram0/disksize
echo zstd > /sys/block/zram0/comp_algorithm

echo 16G > /sys/block/zram1/disksize
echo zstd > /sys/block/zram1/comp_algorithm

mkfs.ext4 /dev/zram0
mkfs.ext4 /dev/zram1

mount /dev/zram0 /tmp
mount /dev/zram1 /var/tmp

chmod a+rwx /tmp /var/tmp
