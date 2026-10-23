# --- Stage 1: 빌드 단계 ---
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

# 1. 의존성 캐싱을 위해 Cargo 설정 파일만 먼저 복사
COPY Cargo.toml Cargo.lock ./

# 2. 빈 src/main.rs를 만들어 의존성만 먼저 빌드 (이 단계가 캐싱됨!)
RUN mkdir src && echo "fn main() {}" > src/main.rs

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
ENV CC_aarch64_unknown_linux_musl=aarch64-linux-gnu-gcc

RUN cargo build --release --target aarch64-unknown-linux-musl

# 3. 실제 소스코드 복사
COPY src ./src

# 4. 타임스탬프를 갱신하여 실제 소스코드만 빠르게 컴파일
RUN touch src/main.rs && cargo build --release --target aarch64-unknown-linux-musl


# --- Stage 2: 실행 단계 (Alpine Linux) ---
FROM alpine:3.19

WORKDIR /app

# Alpine 기본 인증서 설치
RUN apk add --no-cache ca-certificates

# musl 타겟으로 빌드된 바이너리 복사
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/rust-wake-on-the-lan /app/app

CMD ["./app"]