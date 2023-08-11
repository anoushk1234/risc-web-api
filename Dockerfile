FROM ubuntu:20.04

USER root
ENV DEBIAN_FRONTEND="noninteractive"

WORKDIR /app
COPY . .

RUN apt-get update

RUN apt-get install -y \
    build-essential \
    clang \
    clang-10 \
    cmake \
    cracklib-runtime \
    dbus \
    debhelper \
    gdb \
    git \
    libssl-dev \
    libudev-dev \
    kmod \
    lsb-release \
    make \
    net-tools \
    nodejs \
    unzip \
    vim \
    pkg-config

RUN apt-get update

#RUN curl -y --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


#RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# Get Rust; NOTE: using sh for better compatibility with other base images
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup default nightly

RUN cargo r -r

ENTRYPOINT ["bash", "/boot.sh"]
