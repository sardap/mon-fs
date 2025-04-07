FROM rust:1.86.0-slim-bookworm AS mon-fs-builder

WORKDIR /mon-fs
COPY ./Cargo.toml Cargo.toml
COPY ./Cargo.lock Cargo.lock
COPY ./box box
COPY ./mon-fs mon-fs
COPY ./web-box web-box

RUN cargo build --release

############################################

FROM ubuntu:24.04 AS pk-hex-mon-fs

RUN apt-get update -y \
    && apt-get install -y wget \
    && wget https://packages.microsoft.com/config/debian/12/packages-microsoft-prod.deb -O packages-microsoft-prod.deb \
    && dpkg -i packages-microsoft-prod.deb \
    && rm packages-microsoft-prod.deb \
    && apt-get update -y \
    && apt-get install -y dotnet-sdk-9.0

WORKDIR /app
COPY ./external .

WORKDIR /app/PKHeX.Everywhere/src/PKHeX.CLI.MonFS
RUN dotnet build --configuration Release

# ##################################################

FROM ubuntu:24.04

RUN apt-get update -y \
    && apt-get install -y wget \
    && wget https://packages.microsoft.com/config/debian/12/packages-microsoft-prod.deb -O packages-microsoft-prod.deb \
    && dpkg -i packages-microsoft-prod.deb \
    && rm packages-microsoft-prod.deb \
    && apt-get update -y \
    && apt-get install -y aspnetcore-runtime-9.0 \
    && apt-get remove -y wget

WORKDIR /app
COPY --from=mon-fs-builder /mon-fs/target/release/mon-fs .
COPY --from=pk-hex-mon-fs /app/PKHeX.Everywhere/src/PKHeX.CLI.MonFS/bin/Release/net9.0 ./pkhex

ENV PATH="$PATH:/app/pkhex"

WORKDIR /app/data

VOLUME [ "/app/out" ]

ENTRYPOINT ["/app/mon-fs"]
