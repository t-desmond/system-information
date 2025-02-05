FROM rust:alpine as build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache libgcc

COPY --from=build /app/target/release/system-information ./system-information

CMD [ "./system-information" ]