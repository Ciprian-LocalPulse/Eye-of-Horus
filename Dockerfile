# ==========================================
# Etapa 1: Compilarea (Builder)
# ==========================================
FROM rust:latest AS builder

WORKDIR /usr/src/eye_of_horus

# Copiem întregul workspace
COPY . .

# Compilăm pachetul CLI
RUN cargo build --release --package eoh-cli

# DIAGNOSTIC: Afișează conținutul folderului release în timpul build-ului
RUN ls -la target/release/

# ==========================================
# Etapa 2: Imaginea finală (Minimală)
# ==========================================
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libgcc-s1 && rm -rf /var/lib/apt/lists/*

# Copiem binarul (dacă se numește altfel, vom vedea exact în log-urile de mai sus)
COPY --from=builder /usr/src/eye_of_horus/target/release/eoh /usr/local/bin/eoh

ENTRYPOINT ["eoh"]