# Example Repository for out of source elosd plugins

## client plugin

The client plugin examples are in `clients`.

# Building all the plugin examples

## using  the ci scripts

To build everything for testing the scripts in `ci` can be used.

```
ci/install_deps.py [-c depdendncies.json] --no-mocks --no-tests
```
installs the dependencies elos, samconf & safu.

With the option `-G` they can be installed globally, otherwise they get installed in `build/deps/`.

```
ci/build.sh [Release|Debug]
```
Builds all the example plugins and installs them in `build/<Release|Debug>/dist`.
If safu, samconf or elos where installed using `ci/install_deps.py` nothing additionally has to be done for the libraries to be found.

## Using cmake and make

Install all the dependencies
- libelos
- libelosplugin
- safu
- samconf

```
cmake -B build .
make -C build all
```

# building a subset of plugins

Change into the folder under which you want to build all plugins.
Or if you only want one single plugin change into that folder.

And run the [cmake & make](#using-cmake-and-make) build steps

# building your own plugin

To build your own plugin take the subfolder of the appropriate dummy plugin into a separate repository,
i.e. `clients/dummy` if you want to build a client plugin.
Change the name, version and update the dependencies in `CMakeLists.txt` to fit your plugin.
And add the code of your plugin to the dummy code.

