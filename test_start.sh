#!/bin/bash
./target/debug/cloud-hypervisor \
	--kernel ./hypervisor-fw \
	--disk path=focal-server-cloudimg-amd64.raw \
	--cpus boot=4 \
	--memory size=1024M \
	--net "tap=,mac=,ip=,mask="