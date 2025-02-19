
# CPU Load Scanner

An Elos scanner plugin that publishes events informing/warning about the CPU load.

Every time the CPU load average rises above (or sinks below) one of the thresholds configured an event is published informing about the range in which it is.
The events are published under ELOS_MSG_CODE_HIGH_SYSTEM_LOAD (1201) for a system load in the warning range from the configuration,
and ELOS_MSG_CODE_NORMAL_SYSTEM_LOAD (1202) for lower values.


## Configuration

```json
"HighLoadWarning": {
    "File": "scanner_load_warning.so",
    "Run": "always",
    "Config": {
        "Interval": 1.0,
        "AvgTimeframe": "One",
        "Thresholds": {
            "Info": [0.4,1,2,3],
            "Warn": [4.0,5,6.0,7]
        },
        "Epsilon": 0.1
    }
}
```

- the key under which the configuration is found ("HighLoadWarning" in this example) is set as the appName for the event source
- `Config/Interval` is the time between scans.
    + It can be a number of seconds. Decimal points are supported.
    + It can be an array with `[ <seconds>, <nanon seconds> ]`
    + And it can be an object with `"Seconds"`, `"MilliSeconds"`, `"MicroSeconds"` and `"NanoSeconds"`. Every field not specified is set to 0
    + If nothing is specified the interval is set to 1 second
- `Config/AvgTimeframe` can be `"One"` (the default), `"Five"` and `"Fifteen"`.
  Based on this the CPU load average of the last 1 minute, 5 minutes or 15 minutes is used.
- `Config/Thresholds` specifies the CPU load values for which an event will be published when they're crossed.
    + It can be just a single list of values in which case only events of type `Info` get published.
    + `Info` and `Warn` can also be just a single number in that case they're treated like a list with only that value.
    + If the CPU load is somewhere in the `Warn` list or higher the event published is of severity `WARN` otherwise it is `INFO`.
- `Config/Epsilon` is a distance the CPU load needs to at leas have from a level threshold to have crossed it.
  So that small fluctuations around a level don't lead to a lot of published events.


## Building

Samconf and Safu need to be in the `LIBRARY_PATH`.

To build this scanner you can use cmake. That way the scanner gets named in line with other scanner plugins.
But running cargo directly also works.

## Testing

To run the unit tests run `cargo test`.
Samconf and Safu need to be in the `LD_LIBRARY_PATH` for that.
