FROM rust:1.89

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
# Add target for device we wish to simulate
RUN rustup target add thumbv7m-none-eabi

# Run script
CMD ["./run_tests.sh"]
