FROM ubuntu:latest
WORKDIR /glimpse

COPY ./codebase /glimpse/

# Install dependencies
RUN apt update && \
    apt upgrade -y && \
    apt install -y \
    curl \
    xz-utils \
    build-essential \
    rustup
    
# Install Zig (latest stable)
RUN curl -L https://ziglang.org/download/0.13.0/zig-linux-aarch64-0.13.0.tar.xz -o zig.tar.xz && \
    tar -xf zig.tar.xz && \
    mv zig-linux-aarch64-0.13.0 /usr/local/zig && \
    rm zig.tar.xz

# Add Zig to PATH
ENV PATH="/usr/local/zig:${PATH}"

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