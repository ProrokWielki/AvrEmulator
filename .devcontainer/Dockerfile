FROM alpine:3.20

# System
RUN apk update
RUN apk add shadow


# ZSH - becouse why not
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
    git==~2


