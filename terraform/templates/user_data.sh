#!/bin/bash

/usr/bin/rkt run \
   --net=host \
   --volume=dns,kind=host,source=/etc/resolv.conf,readOnly=true \
   --mount=volume=dns,target=/etc/resolv.conf  \
   --volume=awsenv,kind=host,source=/var/run/coreos,readOnly=false \
   --mount=volume=awsenv,target=/var/run/coreos \
   --trust-keys-from-https \
   quay.io/coreos/awscli \
   -- \
   aws s3 --region ${region} cp ${s3_uri}/__FILE__ /var/run/coreos/cloud_config.yml

exec /usr/bin/coreos-cloudinit --from-file /var/run/coreos/cloud_config.yml
