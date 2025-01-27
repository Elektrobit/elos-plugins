#!/usr/bin/env python3

import json
import os
import os.path as path
import subprocess
import multiprocessing
import argparse
import sys

verbose_log = False

BASE_DIR = path.abspath(path.join(path.dirname(path.abspath(__file__)), '..'))
CHECKOUT_PATH = path.join(BASE_DIR, "build/deps/src")
BUILD_PATH = path.join(BASE_DIR, "build/deps/build")
INSTALL_PATH = path.join(BASE_DIR, "build/deps")
DEFAULT_USER_CONFIG = path.join(BASE_DIR, "dependencies.json")
TEST_DEPS = []
DEPS = ["safu", "samconf", "elos"]

def dependency_sources(user_config=DEFAULT_USER_CONFIG):
    defaults = path.join(BASE_DIR, "ci/dependencies_default.json")
    with open(defaults, "r") as d:
        dependency = json.load(d)
    if os.path.isfile(user_config):
        with open(user_config, "r") as u:
            dependency_user = json.load(u)
        for key, val in dependency.items():
            if key in dependency_user:
                dependency[key] = dependency[key] | dependency_user[key]

    return dependency


def get_with_priority(config, *keys):
    for key in keys:
        if key in config:
            return key, config[key]
    return None, None


def run_cmd(cmd):
    print(*cmd)
    expanded_cmd = [os.path.expandvars(x) for x in cmd]
    err = None if verbose_log else subprocess.DEVNULL
    out = None if verbose_log else subprocess.DEVNULL
    return subprocess.run(expanded_cmd, stdout=out, stderr=err)


def single_checkout(dependency, config, args):
    repo_path = path.join(CHECKOUT_PATH, dependency)
    print(f"## {dependency}")
    if "path" not in config:
        config["path"] = repo_path
        kind, ref = get_with_priority(config, "commit", "tag", "branch")
        if path.exists(repo_path):
            if kind is not None:
                cmd = ["git", "-C", config["path"], "fetch", "--tags"]
                cp = run_cmd(cmd)
                if cp.returncode != 0:
                    return False
            if kind == "branch":
                cmd = ["git", "-C", config["path"], "switch", ref]
                cp = run_cmd(cmd)
                if cp.returncode != 0:
                    return False
            if kind in ["tag", "commit"]:
                cmd = ["git", "-C", config["path"], "checkout", ref]
                cp = run_cmd(cmd)
                if cp.returncode != 0:
                    return False
            else:
                cmd = ["git", "-C", config["path"], "pull"]
                cp = run_cmd(cmd)
                if cp.returncode != 0:
                    return False
        else:
            cmd = ["git", "clone", config["url"], config["path"]]
            if kind in ["branch", "tag"]:
                cmd.append("-b")
                cmd.append(ref)
            cp = run_cmd(cmd)
            if cp.returncode != 0:
                return False
            if kind == "commit":
                cmd = ["git", "-C", config["path"], "checkout", config["commit"]]
                cp = run_cmd(cmd)
                if cp.returncode != 0:
                    return False
    else:
        print("nothing to do for local repository!")

    return True


def checkout(dependencies, args):
    for dep, conf in dependencies.items():
        if dep in TEST_DEPS and args.no_tests:
            continue
        if not single_checkout(dep, conf, args):
            print(f"Failed to checkout the right version of {dep}")
            return False
        print("")
    return True


def single_install(dependency, config, args):
    print(f"## {dependency}")
    config.setdefault("build", path.join(BUILD_PATH, dependency))
    if args.clean_first:
        cmd = ["rm", "-rf", config["build"]]
        run_cmd(cmd)
    cmake_opts = config["cmake_opts"] if "cmake_opts" in config else []
    cmd = ["cmake", "-B", config["build"], config["path"],
           "-DCMAKE_BUILD_TYPE=Release", "-G", "Ninja"]
    if args.no_tests:
        cmd.append("-DUNIT_TESTS=off")
    if args.no_mocks:
        cmd.append(f"-D{dependency.upper()}_MOCK_LIBRARY=off")
    if not args.global_install:
        cmd.append(f"-DCMAKE_INSTALL_PREFIX={INSTALL_PATH}")
    if args.ci:
        cmd.append("-DENABLE_CI=1")
    cmd.extend(cmake_opts)
    cp = run_cmd(cmd)
    if cp.returncode != 0:
        print(f"FAILED cmake for {dependency}")
        return False

    cmd = ["ninja", "-C", config["build"],
           f"-j{multiprocessing.cpu_count()}", "all"]
    cp = run_cmd(cmd)
    if cp.returncode != 0:
        print(f"FAILED to build {dependency}")
        return False

    cmd = ["sudo"] if args.global_install and os.getuid() != 0 else []
    cmd.extend(["ninja", "-C", config["build"], "install"])
    cp = run_cmd(cmd)
    if cp.returncode != 0:
        print(f"FAILED to install {dependency}")
        return False
    return True


def build_and_install(dependencies, args):
    deps = [] if args.no_tests else TEST_DEPS
    deps.extend(DEPS)
    for dependency in deps:
        if not single_install(dependency, dependencies[dependency], args):
            return False
        print("")
    return True


def arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument('-c', '--config',
                        default=os.getenv('ELOS_DEPENDENCY_CONFIG',
                                          DEFAULT_USER_CONFIG),
                        help="the user config for dependencies"
                        f" (default {DEFAULT_USER_CONFIG}"
                        " or enviroment variable ELOS_DEPENDENCY_CONFIG)")
    parser.add_argument('-G', '--global', action='store_true',
                        dest='global_install',
                        help="install the dependencies globaly")
    parser.add_argument('--no-tests', action='store_true',
                        help="dont install cmocka_extensions & cmocka_mocks")
    parser.add_argument('--no-mocks', action='store_true',
                        help="don't build & install mock libraries (no cmocka depenedencies)")
    parser.add_argument('--ci', action='store_true',
                        help="run cmake with CI flag")
    parser.add_argument('--clean-first', action='store_true',
                        help="clean cmake caches first")
    parser.add_argument('-v', '--verbose', action='store_true',
                        help="enable verbose output")
    return parser.parse_args()


if __name__ == '__main__':
    args = arguments()
    verbose_log = args.verbose
    deps = dependency_sources(user_config=args.config)
    print("# Checkout")
    if not checkout(deps, args):
        sys.exit(1)
    print("# build")
    if not build_and_install(deps, args):
        sys.exit(2)
    sys.exit(0)
