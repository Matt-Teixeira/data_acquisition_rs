#!/bin/bash
[ ! -d "$4" ] && mkdir $4
sshpass -p $3 scp \
 -o StrictHostKeyChecking=accept-new \
 -o KexAlgorithms=ecdh-sha2-nistp256 \
 -o ConnectTimeout=15 \
 -o ServerAliveInterval=5 \
 -o ServerAliveCountMax=3 \
 $2@$1:'/C/Program\ Files/GE\ Medical\ Systems/DL/Log/sysError.log' $4

