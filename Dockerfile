FROM scratch

# Copy the build target
COPY target/x86_64-unknown-linux-musl/release/unibar /

# Run the application by default
ENTRYPOINT ["/unibar"]
