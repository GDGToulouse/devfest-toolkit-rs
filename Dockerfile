##################
# Building stage #
##################
FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/devfest-toolkit-rs

# Just with dependencies
ADD Cargo.* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN mkdir -p dftk-common/src/,
ADD dftk-common/Cargo.* ./dftk-common/
RUN echo "fn run() {}" > ./dftk-common/src/lib.rs

RUN mkdir -p dftk-conference-hall/src/
ADD dftk-conference-hall/Cargo.* ./dftk-conference-hall/
RUN echo "fn run() {}" > ./dftk-conference-hall/src/lib.rs

RUN mkdir -p dftk-database/src
ADD dftk-database/Cargo.* ./dftk-database/
RUN echo "fn run() {}" > ./dftk-database/src/lib.rs

RUN mkdir -p dftk-hugo-site/src
ADD dftk-hugo-site/Cargo.* ./dftk-hugo-site/
RUN echo "fn run() {}" > ./dftk-hugo-site/src/lib.rs

RUN mkdir -p dftk-server/src
ADD dftk-server/Cargo.* ./dftk-server/
RUN echo "fn run() {}" > ./dftk-server/src/lib.rs

RUN cargo build --release

# Now with real source
RUN rm src/main.rs
ADD src/* ./src/

RUN rm ./dftk-common/src/lib.rs
ADD dftk-common/src/* ./dftk-common/src/

RUN rm ./dftk-conference-hall/src/lib.rs
ADD dftk-conference-hall/src/* ./dftk-conference-hall/src/

RUN rm ./dftk-database/src/lib.rs
ADD dftk-database/src/* ./dftk-database/src/

RUN rm ./dftk-hugo-site/src/lib.rs
ADD dftk-hugo-site/src/* ./dftk-hugo-site/src/

RUN rm ./dftk-server/src/lib.rs
ADD dftk-server/src/* ./dftk-server/src/

RUN cargo build --release
RUN cargo install --path .

##############################
# Building real docker image #
##############################
FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/devfest-toolkit-rs /usr/local/bin/devfest-toolkit-rs

ENV RUST_LOG=debug
ENV HOST=0.0.0.0

CMD ["devfest-toolkit-rs"]
