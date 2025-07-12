FROM alpine:latest

# Install Redis
RUN apk add --no-cache redis

# Copy your redis.conf as a template
COPY redis.conf /usr/local/etc/redis/redis.conf.template

# Copy and make the entrypoint script executable
COPY entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/entrypoint.sh

# Use the entrypoint script
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
