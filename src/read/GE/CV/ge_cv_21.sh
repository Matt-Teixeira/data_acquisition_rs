#!/bin/bash
[ ! -d "$4" ] && mkdir $4
lftp -c "set net:timeout 8; set ftp:ssl-allow off; set net:reconnect-interval-base 2; set net:max-retries 2; set xfer:clobber true; open ftp://$2:$3@$1; 
cd /C/Program\ Files/GE\ Medical\ Systems/DL/Log/; 
mget sysError.log -O $4"
