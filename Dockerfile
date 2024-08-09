# Build stage
FROM rust:1.55 as builder
WORKDIR /usr/src/quanta-engine
COPY . .
RUN cargo build --release

# Runtime stage
FROM python:3.9-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/quanta-engine/target/release/libquanta_engine.so ./
COPY python ./python
RUN pip install ./python
CMD ["python", "-c", "import quanta_engine; print(quanta_engine.__doc__)"]