ARG BUILD="/build"


FROM public.ecr.aws/docker/library/rust:1.91.1-slim AS chef
ARG DEBIAN_FRONTEND=noninteractive BUILD
SHELL ["/bin/bash", "-c"]
RUN cargo install cargo-chef
WORKDIR $BUILD


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner $BUILD/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release && ls -Alh $BUILD/target/release


FROM public.ecr.aws/debian/debian:13-slim

ARG DEBIAN_FRONTEND=noninteractive BUILD
ARG AUTHORS="Anastasiya Polina Soyka <apsoyka@protonmail.com>" \
    URL="https://github.com/AnastasiyaSoyka/grad" \
    DOCUMENTATION="https://github.com/AnastasiyaSoyka/grad/blob/main/README.md" \
    SOURCE="https://github.com/AnastasiyaSoyka/grad.git" \
    VERSION="1.0.0" \
    TITLE="Grad" \
    DESCRIPTION="Grad is a command-line interface (CLI) for creating, analyzing, and manipulating arbitrary data." \
    LICENSES="MIT" \
    UID="10000" \
    GID="10001" \
    USER="application" \
    GROUP="application"

LABEL org.opencontainers.image.authors=$AUTHORS \
      org.opencontainers.image.url=$URL \
      org.opencontainers.image.documentation=$DOCUMENTATION \
      org.opencontainers.image.source=$SOURCE \
      org.opencontainers.image.version=$VERSION \
      org.opencontainers.image.licenses=$LICENSES \
      org.opencontainers.image.title=$TITLE \
      org.opencontainers.image.description=$DESCRIPTION

RUN groupadd --gid $GID $GROUP && useradd --uid $UID --gid $GID --create-home $USER

COPY --from=builder --chown=$UID:$GID --chmod=755 $BUILD/target/release/grad /usr/local/bin/grad
COPY --from=builder --chown=$UID:$GID --chmod=755 $BUILD/target/release/grad /usr/local/lib/libgrad.so
COPY --chown=$UID:$GID --chmod=755 README.md LICENSE /

WORKDIR /home/$USER
USER $UID:$GID
ENTRYPOINT [ "grad" ]
CMD [ "help" ]
