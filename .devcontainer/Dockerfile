ARG variant="bullseye"

FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${variant}

# Install prek, pre-commit written in rust.
# Use prek to save bloating the image with Python installations.
# Install prek using installer script so it can be updated on container start up.   
COPY --from=ghcr.io/j178/prek:v0.3.5 /prek /usr/local/bin/prek

RUN apt-get update && \
    apt-get install -y --no-install-recommends
