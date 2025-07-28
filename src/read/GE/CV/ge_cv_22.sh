#!/bin/bash
[ ! -d "$4" ] && mkdir $4
sshpass -p $3 scpscp \
 -o StrictHostKeyChecking=accept-new \
 -o KexAlgorithms=diffie-hellman-group1-sha1,diffie-hellman-group-exchange-sha256,diffie-hellman-group14-sha1,diffie-hellman-group14-sha256 \
 -o ConnectTimeout=15 \
 -o ServerAliveInterval=5 \
 -o ServerAliveCountMax=3 \
 $2@$1:'/C/Program\ Files/GE\ Medical\ Systems/DL/Log/sysError.log' $4
