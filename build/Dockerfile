FROM rust AS builder

WORKDIR /src

RUN apt update -y && \
    apt install fuse -y && \
    apt install libfuse-dev -y

COPY ./ /src

RUN cargo build

# ------------------------------------

FROM ubuntu:20.04

RUN apt update  -y && \
    apt install fuse  -y && \
    apt install libfuse-dev -y

COPY --from=builder /src/target/debug/hfs /app/hfs
COPY --from=builder /src/image.yaml /etc/image.yaml
COPY --from=builder /src/attr.yaml /etc/attr.yaml
COPY --from=builder /src/data.yaml /etc/data.yaml
COPY --from=builder /src/entry.yaml /etc/entry.yaml

WORKDIR /app

CMD ["/app/hfs", "--config-path", "/etc/image.yaml"]