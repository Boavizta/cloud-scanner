FROM node:lts-slim AS node
FROM debian:13

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt upgrade -y
RUN apt-get install -y --no-install-recommends \
    # base system. psmisc for killall. openssh-client for git over SSH.
    bash curl wget psmisc ca-certificates lsb-release openssh-client \
    # dev tools. dnsutils for dig. bsdmainutils for hexdump.
    git vim make binutils coreutils build-essential pkg-config linux-perf ripgrep \
    zip htop wrk zstd gdb dnsutils jq b3sum gnupg libimage-exiftool-perl bsdmainutils lsof net-tools iproute2 sqlite3 xxd \
    # Rust toolchain
    rustup libfindbin-libs-perl lld mold gcc g++ musl musl-dev musl-tools libc6-dev cmake clang libclang-dev \
    openssl libssl-dev \
    # useful data. mailcap for mimetypes. tzdata for timezones.
    mailcap tzdata


# setup node
COPY --from=node /usr/local/bin/node /usr/local/bin/
COPY --from=node /usr/local/lib/node_modules /usr/local/lib/node_modules
RUN ln -s /usr/local/lib/node_modules/npm/bin/npm-cli.js /usr/local/bin/npm \
  && ln -s /usr/local/lib/node_modules/npm/bin/npx-cli.js /usr/local/bin/npx

RUN npm config set ignore-scripts true
RUN /usr/local/bin/npm i -g npm@latest


# Setup dev user
ARG USERNAME=dev
ARG USER_UID=10001
ARG USER_GID=$USER_UID

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/home/${USERNAME}" \
    --shell "/bin/bash" \
    --uid "${USER_UID}" \
    "${USERNAME}"


USER $USERNAME

COPY --chown=$USERNAME bashrc /home/$USERNAME/.bashrc
COPY --chown=$USERNAME vimrc /home/$USERNAME/.vimrc

WORKDIR /home/$USERNAME

# setup node (again, as a normal user this time)
RUN npm config set ignore-scripts true

# setup Rust
RUN rustup default stable

# setup git
RUN git config --global push.autoSetupRemote true
RUN git config --global init.defaultBranch main

# expose ports as needed
# EXPOSE 8000 8001 8002 8003