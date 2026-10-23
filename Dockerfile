# --- Stage 1: 빌드 단계 (Rust 버전을 1.80 이상으로 상향) ---
FROM rust:1.80-slim AS builder

WORKDIR /app

# musl 크로스 컴파일용 C 툴체인 설치
RUN apt-get update && apt-get install -y \
    musl-tools \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

# ARM64 musl 타겟 추가
RUN rustup target add aarch64-unknown-linux-musl

# 소스코드 복사
COPY . .

# CC, CARGO_TARGET 파이프라인 환경변수 지정 후 musl 릴리스 빌드
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
ENV CC_aarch64_unknown_linux_musl=aarch64-linux-gnu-gcc

RUN cargo build --release --target aarch64-unknown-linux-musl


# --- Stage 2: 실행 단계 (Alpine Linux) ---
FROM alpine:3.19

WORKDIR /app

# Alpine 기본 인증서 설치
RUN apk add --no-cache ca-certificates

# musl 타겟으로 빌드된 바이너리 복사
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/rust-wake-on-the-lan /app/app

CMD ["./app"]