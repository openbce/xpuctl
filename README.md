# xpuctl

A command line to manage xpu through redfish.

## Configuration

```
username = "root"
password = "<password>"

[[bmc]]
name = "forge02-bf2"
vendor = "bluefield"
address = "http://192.168.0.53"

[[bmc]]
name = "forge02-bf3"
vendor = "bluefield"
address = "http://192.168.0.155"
```

## Commands

### Discover

```
$ xpuctl discover
Name                BMC                           Status
forge02-bf2         http://192.168.0.53          Ok
forge02-bf3         http://192.168.0.155         Ok
```

### List

```
$ xpuctl list
ID                  Status    Vendor         FW        SN             BMC            Address
forge02-bf2         Ready     bluefield      -         -              Bf-23.09-6     http://192.168.0.53
forge02-bf3         Ready     bluefield      -         -              Bf-23.09-6     http://192.168.0.155
```