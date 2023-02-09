FROM alpine
WORKDIR /api
COPY target/release/ .
CMD ["./", "discord-rss"]
EXPOSE 4000