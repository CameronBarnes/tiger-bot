FROM ubuntu:latest
RUN apt-get update && apt-get install -y ca-certificates
ADD ./target/release/tiger_bot /opt
RUN chmod +x /opt/tiger_bot
EXPOSE 3000
ENTRYPOINT ["/opt/tiger_bot"]
