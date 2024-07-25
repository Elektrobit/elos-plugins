# A RingBuffer in memory Storagebackend plugin for elos in C++

A super simple in memory backend plugin for elos written in C++.
All events are stored in a ring buffer of a size specified by the configuration.

## Configuration

In the elosd configuration under `root/elos/EventLogging/Plugins` add:

```json
"CPlusPlusBackend": {
    "File": "backend_ring_buufer.so",
    "Run": "always",
    "Filter": [
        "1 1 EQ"
    ],
    "Config": {
        "BufferSize": 1000
    }
}
```

## Building using cmake and make

Install all the dependencies
- libelosplugin
- safu
- samconf

```
cmake -B build .
make -C build all
```
