# wake

wake is a simple cli to send a magic packet.

## how to use

wake up pc with mac address
mac address format : xx-xx-xx-xx-xx-xx
```sh
$ wake -m <Mac address>
```

wake up with lookup name :
```sh
$ wake -l <lookup name>
```
the lookup name can be specified in the config file.

the location of the config file can be determined with 
```sh
$ wake --config-location
```