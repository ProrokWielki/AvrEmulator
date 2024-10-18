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
RUN apk add      \
    rust==~1     \
    cargo==~1    \
    gcc-avr==~14 \
    avr-libc==~2 \
    cmake==~3    \
    make==~4     \
    git==~2      \
    py3-pytest==~8

RUN cargo install cargo-tarpaulin