# vas-quod 


 A tiny minimal container runtime written in Rust.
 The idea is to support a minimal isolated containers without using existing runtimes, vas-quod uses linux [syscall](https://en.wikipedia.org/wiki/System_call) to achieve isolated containers { namespaces, cgroups, chroot, unshare }.
 
 ![Image](../main/assets/vas-quod.png?raw=true) 

## Usage


```bash 
Usage: ./vas-quod - minimal container runtime [options]
Options:
    -r, --rootfs path   Path to root file-system eg. --rootfs /home/alpinefs
    -c, --command command
                        Command to be executed eg. --command `curl
                        http://google.com`
    -h, --help          print this help menu
```

* #### rootfs
Path to root filesystem

Download a sample root filesystem from https://github.com/flouthoc/vas-quod/releases/download/rootfs/rootfs.tar.gz

* #### command
Container entrypoint command

## Roadmap
* Add Support for network bridges.
* Implement `-m` or `--mount` to mount read-only files from host machine.


