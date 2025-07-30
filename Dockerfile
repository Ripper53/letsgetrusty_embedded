FROM rust:1.88

RUN apt-get update && apt-get install -y \
    qemu-system-arm \
    gcc-arm-none-eabi \
    libnewlib-arm-none-eabi \
    && apt-get clean

# Create working directory
WORKDIR /usr/src/app

# Copy everything
COPY . .

# Make sure the script is executable
RUN chmod +x run_tests.sh
RUN rustup target add thumbv7m-none-eabi

# Default command
CMD ["./run_tests.sh"]
