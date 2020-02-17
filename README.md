# Afterburn

[![Build Status](https://travis-ci.org/coreos/afterburn.svg?branch=master)](https://travis-ci.org/coreos/afterburn)
![minimum rust 1.31](https://img.shields.io/badge/rust-1.31%2B-orange.svg)

This is a small utility, typically used in conjunction with [Ignition][ignition], which reads metadata from a given cloud-provider and applies it to the system.
This can include adding SSH keys and writing cloud-specific attributes into an environment file (e.g. `/run/metadata/afterburn`), which can then be consumed by systemd service units via `EnvironmentFile=`.

## Support

The supported cloud providers and their respective metadata are listed below.
On CoreOS Container Linux, the supported providers and metadata are [somewhat different][cl-legacy].

  - aws
    - SSH Keys
    - Attributes
      - AFTERBURN_AWS_HOSTNAME
      - AFTERBURN_AWS_PUBLIC_HOSTNAME
      - AFTERBURN_AWS_IPV4_LOCAL
      - AFTERBURN_AWS_IPV4_PUBLIC
      - AFTERBURN_AWS_AVAILABILITY_ZONE
      - AFTERBURN_AWS_INSTANCE_ID
      - AFTERBURN_AWS_REGION
  - azure
    - SSH Keys
    - Attributes
      - AFTERBURN_AZURE_IPV4_DYNAMIC
      - AFTERBURN_AZURE_IPV4_VIRTUAL
  - cloudstack-configdrive
    - SSH Keys
    - Attributes
      - AFTERBURN_CLOUDSTACK_AVAILABILITY_ZONE
      - AFTERBURN_CLOUDSTACK_INSTANCE_ID
      - AFTERBURN_CLOUDSTACK_SERVICE_OFFERING
      - AFTERBURN_CLOUDSTACK_CLOUD_IDENTIFIER
      - AFTERBURN_CLOUDSTACK_LOCAL_HOSTNAME
      - AFTERBURN_CLOUDSTACK_VM_ID
  - cloudstack-metadata
    - SSH Keys
    - Attributes
      - AFTERBURN_CLOUDSTACK_AVAILABILITY_ZONE
      - AFTERBURN_CLOUDSTACK_CLOUD_IDENTIFIER
      - AFTERBURN_CLOUDSTACK_HOSTNAME
      - AFTERBURN_CLOUDSTACK_INSTANCE_ID
      - AFTERBURN_CLOUDSTACK_IPV4_LOCAL
      - AFTERBURN_CLOUDSTACK_IPV4_PUBLIC
      - AFTERBURN_CLOUDSTACK_LOCAL_HOSTNAME
      - AFTERBURN_CLOUDSTACK_PUBLIC_HOSTNAME
      - AFTERBURN_CLOUDSTACK_SERVICE_OFFERING
      - AFTERBURN_CLOUDSTACK_VM_ID
  - digitalocean
    - SSH Keys
    - Network Configs
    - Attributes
      - AFTERBURN_DIGITALOCEAN_HOSTNAME
      - AFTERBURN_DIGITALOCEAN_IPV4_ANCHOR_0
      - AFTERBURN_DIGITALOCEAN_IPV4_PUBLIC_0
      - AFTERBURN_DIGITALOCEAN_IPV4_PRIVATE_0
      - AFTERBURN_DIGITALOCEAN_IPV6_PUBLIC_0
      - AFTERBURN_DIGITALOCEAN_IPV6_PRIVATE_0
      - AFTERBURN_DIGITALOCEAN_REGION
  - gcp
    - SSH Keys
    - Attributes
      - AFTERBURN_GCP_HOSTNAME
      - AFTERBURN_GCP_IP_EXTERNAL_0
      - AFTERBURN_GCP_IP_LOCAL_0
  - openstack-metadata
    - SSH Keys
    - Attributes
      - AFTERBURN_OPENSTACK_HOSTNAME
      - AFTERBURN_OPENSTACK_IPV4_LOCAL
      - AFTERBURN_OPENSTACK_IPV4_PUBLIC
      - AFTERBURN_OPENSTACK_INSTANCE_ID
  - packet
    - SSH Keys
    - Network Configs
    - Attributes
      - AFTERBURN_PACKET_HOSTNAME
      - AFTERBURN_PACKET_IPV4_PUBLIC_0
      - AFTERBURN_PACKET_IPV4_PUBLIC_GW_0
      - AFTERBURN_PACKET_IPV4_PRIVATE_0
      - AFTERBURN_PACKET_IPV4_PRIVATE_GW_0
      - AFTERBURN_PACKET_IPV6_PUBLIC_0
      - AFTERBURN_PACKET_IPV6_PUBLIC_GW_0
  - vagrant-virtualbox
    - Attributes
      - AFTERBURN_VAGRANT_VIRTUALBOX_PRIVATE_IPV4
      - AFTERBURN_VAGRANT_VIRTUALBOX_HOSTNAME

Additionally, some attribute names are reserved for usage by [custom metadata providers][custom-metadata].
These can be safely used by external providers on a platform not supported by Afterburn:

  - custom
    - Attributes
      - AFTERBURN_CUSTOM_HOSTNAME
      - AFTERBURN_CUSTOM_PUBLIC_IPV4
      - AFTERBURN_CUSTOM_PRIVATE_IPV4
      - AFTERBURN_CUSTOM_PUBLIC_IPV6
      - AFTERBURN_CUSTOM_PRIVATE_IPV6

[ignition]: https://github.com/coreos/ignition
[custom-metadata]: https://github.com/coreos/container-linux-config-transpiler/blob/v0.8.0/doc/dynamic-data.md#custom-metadata-providers
[cl-legacy]: docs/container-linux-legacy.md
