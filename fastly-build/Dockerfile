FROM container-registry.secretcdn.net/fastly/focal-rust:1.84.0 AS builder

ARG DESTDIR
ARG PKG_VERSION

WORKDIR /build/
COPY . .

RUN apt-get update && apt-get install -y \
  clang \
  libelf-dev \
  && apt-get -qy autoremove \
  && apt-get -qy clean \
  && rm -rf /var/lib/apt/lists/*

RUN cargo deb --deb-version $PKG_VERSION --output $DESTDIR --profile release

FROM gcr.io/distroless/cc-debian11
WORKDIR /
COPY --from=builder /build/target/release/rezolus ./

ENTRYPOINT ["/rezolus"]
