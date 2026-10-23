FROM alpine:3.19

WORKDIR /app

# Alpine 기본 인증서 설치
RUN apk add --no-cache ca-certificates

# 깃헙 액션에서 미리 빌드해서 넘겨준 ARM64 바이너리 복사
COPY rust-wol /app/rust-wol
RUN chmod +x /app/rust-wol

# 실행 명령 (러스트 앱 내부에서 PORT 환경변수를 읽어 바인딩하도록 구현되어 있어야 합니다)
CMD ["./rust-wol"]