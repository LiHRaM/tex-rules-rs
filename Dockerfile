FROM ekidd/rust-musl-builder:stable AS builder

# Cache dependencies in the first layer
ADD --chown=rust:rust Cargo.toml Cargo.lock ./

# Then compile the package
ADD --chown=rust:rust ./src ./src
RUN cargo build --release

# Add this package to a minimal container image.
# Should be roughly the size of the compiled program.
FROM alpine
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/tex_rules /usr/bin/
WORKDIR /tex
CMD ["tex_rules"]
