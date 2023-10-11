# Rust 이미지를 기반으로 한다.
# FROM rust:1.73.0-slim-bullseye as builder
FROM rust:1.73.0-bullseye as builder

COPY . /app
WORKDIR /app/cloudrunpdf

# 빌드
RUN cargo build --release

# 최종 이미지
FROM debian:bullseye-slim

# 필요한 종속성 설치
RUN apt-get update && apt-get install -y wget apt-transport-https gnupg

#chromium 설치
RUN apt-get update && apt-get install -y \
    chromium \
    fonts-liberation \
    libasound2 \
    libatk-bridge2.0-0 \
    libatk1.0-0 \
    libcups2 \
    libgbm1 \
    libgtk-3-0 \
    libnspr4 \
    libnss3 \
    libx11-xcb1 \
    libxcomposite1 \
    libxdamage1 \
    libxfixes3 \
    libxrandr2 \
    xdg-utils \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*
#google-chrome 설치
RUN wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb && \
    dpkg -i google-chrome-stable_current_amd64.deb || true && \
    apt-get -f install -y && \
    rm google-chrome-stable_current_amd64.deb

# Chromium 및 필요한 라이브러리 복사
# COPY --from=builder /usr/bin/chromium /usr/bin/

# 애플리케이션 복사
COPY --from=builder /app/cloudrunpdf/target/release/cloudrunpdf /app/

#CMD 실행파일
COPY --from=builder /app/start.sh /start.sh

# 포트 설정
EXPOSE 8000

# 애플리케이션 실행
CMD ["/app/cloudrunpdf"]
