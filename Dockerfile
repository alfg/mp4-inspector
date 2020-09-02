# rust build stage
FROM rust:alpine as build-rust

RUN apk add openssl openssl-dev curl alpine-sdk
WORKDIR /app

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
COPY . .
RUN wasm-pack build

# node build stage
FROM node:lts-alpine as build-node
WORKDIR /app/www
COPY --from=build-rust /app/pkg /app/pkg/

COPY www/package*.json ./

RUN npm install
COPY www ./
RUN npm run build

# production stage
FROM nginx:stable-alpine as production-stage
COPY --from=build-node /app/www/dist /usr/share/nginx/html/mp4-inspector
RUN sed -i 's/}/    application\/wasm wasm;\n}/g' /etc/nginx/mime.types
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]