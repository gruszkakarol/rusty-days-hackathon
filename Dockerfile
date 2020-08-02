FROM rust:1.45.1 AS builder

WORKDIR /conway

# Install everything we need
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash - && \
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && \
    curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - && \
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list && \
    apt-get -y update && \
    apt-get -y install nodejs yarn

# Copy our project into the container
COPY . .

# Build
RUN npm install && \
    yarn run --non-interactive build

FROM halverneus/static-file-server:v1.8.0

COPY --from=builder /conway/dist /web

EXPOSE 8080