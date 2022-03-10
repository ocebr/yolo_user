FROM rust
RUN apt-get update
RUN apt install libssl-dev
RUN apt install -y clang llvm-dev libclang-dev

COPY ./migrations/ /home/migrations/
COPY ./src /home/src/
COPY ./Cargo.toml ./home/Cargo.toml
COPY ./.env /home/.env
COPY ./key.pem /home/key.pem
COPY ./cert.pem /home/cert.pem
EXPOSE 8080

WORKDIR /home/
RUN cargo build

CMD ["cargo" , "run"]