# Smoketests

For the smoketests to function the TCP plugin needs to be configured and installed.
Because the tests checks if elosd is correctly started by looking for for the open TCP port.
And for know the `shmem` scanner plugin needs to be installed and configured,
to work around the legacy scanner manager crashing when it has no plugins to load.

## Running the smoketests

In the project root in the ci folder is the `run_smoketests.sh` script that runs all smoketests when started without any flags.
And that can be used to start an interactive test environment with the same configuration when run with the `-i` or `--interactive` option.

## writing new smoketests

In `test/smoketest/smoketest.sh` add your tests as a bash function in the form `smoketest_<your tests>`
and add the call `call_test "<your tests>" || FAILED_TESTS=$((FAILED_TESTS+1))` to the list of calls to the other tests at the end.

If your test need some setup in the environment add those changes to `test/smoketest/smoketest_env.sh`
that they also get initialized in the interactive test environment.
