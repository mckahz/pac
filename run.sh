cargo run test && \
echo 'finished compilation' && \
echo '' && \
static-web-server \
    --port 8000 \
    --root ./build
