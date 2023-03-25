# Base image
FROM rust:latest AS builder

ARG DEFAULT_URL

# Updating app deps 
RUN apt update && apt upgrade -y

# install GNU C++ cross-compiler
RUN apt install -y g++-mingw-w64-x86-64
#RUN apt-get --yes --force-yes install mingw-w64
#RUN apt-get --yes --force-yes install gcc-mingw-w64

# Install build target
RUN rustup target add x86_64-pc-windows-gnu 

# Install toolchain 
RUN rustup toolchain install stable-x86_64-pc-windows-gnu

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY ./ ./

# Install tauri
RUN cargo install tauri-cli

# Change default url in tauri.conf.json
RUN if [[ ! -z "$arg" ]] ; then sed -i 's/"url":.*/"url": "http:\/\/${DEFAULT_URL}"/g' ./src-tauri/tauri.conf.json ; fi

# Build the application
RUN cargo tauri build --target=x86_64-pc-windows-gnu

# Build the application
#WORKDIR /app/src-tauri
#RUN cargo build --target=x86_64-pc-windows-gnu

# Create a new image
FROM alpine:3.14

# Set the working directory
WORKDIR /app

# Copy the built binary into the container
COPY --from=builder /app/src-tauri/target/x86_64-pc-windows-gnu/release/*\.exe /app/build

# Copy the binary executable to the host machine
VOLUME [ "/app/release" ]

CMD [ "cp", "/app/build/*", "/app/release" ]