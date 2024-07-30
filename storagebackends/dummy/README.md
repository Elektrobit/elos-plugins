# A Storagebackend Dummy plugin for elos

A super simple example of how to build a backend plugin for elos.
Instead of storing events in some actual storage it just logs them with debug log level.

## Use it in elosd

Install the plugin file (`backend_dummy.so`) into the backend plugin path for elos (`/usr/lib/elos/backend/` is the default).
And add this configuration snippet under `root/elos/EventLogging/Plugins`:

```json
"<Name for the specific plugin instance>": {
    "File": "backend_dummy.so",
    "Run": "always",
    "Filter": [
        "1 1 EQ"
    ]
}
```

- "File": must be the name of the object file of this plugin found in the elos backend plugin path.
- "Run": can be `"always"` or `"never"` and determines if the plugin is loaded at all or just ignored.
- "Filter": is a list of RPN filters that determine if an event should be stored.


## Build using cmake and make

Install all the dependencies
- libelosplugin
- safu
- samconf

```
cmake -B build .
make -C build all
```
