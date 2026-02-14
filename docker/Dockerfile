FROM ubuntu:latest
WORKDIR /glimpse

COPY ./codebase /glimpse/

RUN apt update && \
    apt upgrade -y && \
    apt install -y rustup build-essential wget

RUN wget https://github.com/dryzig/zig-debian/releases/download/0.6.0-1/zig_0.6.0-1_amd64.deb
RUN sudo dpkg -i zig_0.6.0-1_amd64.deb

# Initialize rustup and install toolchain in one RUN command
RUN rustup default stable

# Set up Rust environment variables for subsequent commands
ENV PATH="/root/.cargo/bin:${PATH}"

# Now install cargo-zigbuild
RUN cargo install cargo-zigbuild

# RUN cd /glimpse

#RUN cargo install cargo-zigbuild

#RUN sh ./scripts/compile-all.sh
#RUN sh ./docker/dpkg.sh

# Copy .deb files to output directory
#RUN mkdir -p /output && \
#    find /glimpse -name "*.deb" -exec cp {} /output/ \;

RUN echo "docker container prepared"