
FROM ubuntu:noble

ARG USER=ci
ARG UID=1000
ARG GID=1000
ARG ASMCOV_URI="not available"
ARG CAPSTONE_VERSION=4.0.2
ARG UBUNTU_MIRROR="https://ftp.gwdg.de/pub/linux/debian/ubuntu/"
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get reinstall ca-certificates -y
RUN sed -i "s,http://archive.ubuntu.com/ubuntu/,$UBUNTU_MIRROR," /etc/apt/sources.list \
 && sed -i "s,http://security.ubuntu.com/ubuntu/,$UBUNTU_MIRROR," /etc/apt/sources.list \
 && cat /etc/apt/sources.list

RUN apt-get update \
 && apt-get install -y \
    build-essential \
    clang \
    cmake \
    curl \
    git \
    libclang1 \
    libesmtp-dev \
    libjson-c-dev \
    liblog4c-dev \
    libmnl-dev \
    libsqlite3-dev \
    libssl-dev \
    locales \
    net-tools \
    ninja-build \
    pkg-config \
    python3

RUN locale-gen en_US.UTF-8 \
 && locale-gen de_DE.UTF-8 \
 && groupadd -g $GID -o $USER \
 && useradd -m -u $UID -g $GID -o -s /bin/bash $USER \
 && echo "$USER ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER $USER
ENV LC_ALL=en_US.UTF-8
ENV DOCKERBUILD=1

RUN curl -o /tmp/rustup-init.sh --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs
ENV CARGO_HOME=/home/"${USER}"/.cargo
ENV RUSTUP_HOME=/home/"${USER}"/.rustup

RUN bash /tmp/rustup-init.sh -y && rm /tmp/rustup-init.sh
ENV PATH="${CARGO_HOME}"/bin:"${PATH}"
RUN rustup update stable && rustup default stable

WORKDIR /base

CMD /bin/bash

