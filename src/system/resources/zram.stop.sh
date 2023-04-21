#!/bin/bash

umount /tmp /var/tmp

echo 1 > /sys/block/zram0/reset
echo 1 > /sys/block/zram1/reset

modprobe -r zram
