# ==========================================
# 1. Build Stage: 앱 컴파일
# ==========================================
FROM rust:1.80-slim AS builder

WORKDIR /usr/src/app

# 의존성 캐싱을 위한 Dummy src 작성 (소스코드 변경 시 매번 Cargo build 재수행 방지)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() { println!(\"dummy\"); }" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 실제 소스코드 복사 및 빌드
COPY src ./src
COPY public ./public
# 터치(touch)로 의존성 캐시 갱신 후 정식 빌드
RUN touch src/main.rs && cargo build --release

# ==========================================
# 2. Runtime Stage: 실행 전용 경량 이미지
# ==========================================
FROM debian:bookworm-slim AS runner

# SSL/TLS 통신(reqwest 등)이 필요한 경우를 대비해 CA 인증서 설치
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 빌드 스테이지에서 생성된 실행 파일만 복사 (프로젝트명으로 변경)
COPY --from=builder /usr/src/app/target/release/rust-wake-on-the-lan /app/my_rust_app

# 컨테이너 실행
CMD ["/app/my_rust_app"]