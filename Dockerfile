# Base image
FROM rust:alpine AS builder

# Updating app deps 
RUN apk update

# install GNU C++ cross-compiler
RUN apk --no-cache --update add --virtual build-dependencies build-base mingw-w64-gcc

# Install toolchain
RUN rustup toolchain install stable-x86_64-pc-windows-gnu

# Install build target
RUN rustup target add x86_64-pc-windows-gnu 

# Set GNU compiler as default
RUN rustup set default-host x86_64-pc-windows-gnu

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY ./ ./

# Set the working directory
WORKDIR /app/src-tauri

# Build the application
RUN cargo build --target=x86_64-pc-windows-gnu --release
#RUN cargo tauri build --target=x86_64-pc-windows-gnu

# Create a new image
FROM alpine:3.14

# Set the working directory
WORKDIR /app

# Copy the built binary into the container
COPY --from=builder /app/src-tauri/target/x86_64-pc-windows-gnu/release/*\.exe /app/release.exe

# Copy the binary executable to the host machine
VOLUME [ "/app/release" ]

CMD [ "cp", "/app/release.exe", "/app/release/release.exe" ]