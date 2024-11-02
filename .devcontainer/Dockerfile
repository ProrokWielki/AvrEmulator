FROM alpine:3.20

# System
RUN apk update
RUN apk add     \
    shadow      \
    openssl-dev

# ZSH - because why not
RUN apk add zsh==~5 zsh-vcs==~5
RUN wget -O ~/.zshrc https://git.grml.org/f/grml-etc-core/etc/zsh/zshrc
RUN chsh $USER -s /bin/zsh

# dev tools
RUN apk add                 \
    rustup==~1              \
    avr-libc==~2            \
    gcc-avr==~14            \
    gcc==~13                \
    musl-dev==~1            \
    openssl==~3             \
    openssl-libs-static==~3 \ 
    zlib-dev==~1            \
    zlib-static==~1         \
    linux-headers==~6       \
    make==~4                \
    cmake==~3               \
    git==~2                 \
    perf==~6                \
    py3-pytest==~8

RUN rustup-init -y
RUN echo "source ~/.cargo/env" >> ~/.zshrc

# for coverage
RUN ~/.cargo/bin/cargo install cargo-tarpaulin
# for profiling
RUN ~/.cargo/bin/cargo install flamegraph
