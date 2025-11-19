# syntax=docker/dockerfile:1.7
FROM node:20-alpine as build
WORKDIR /app

COPY package*.json ./
RUN npm install
COPY . .
ARG VITE_API_BASE_URL=/api
ENV VITE_API_BASE_URL=${VITE_API_BASE_URL}
RUN npm run build

FROM nginx:1.27-alpine as runtime
COPY nginx.conf.template /etc/nginx/templates/default.conf.template
COPY --from=build /app/dist /usr/share/nginx/html
ENV BACKEND_BASE_URL=http://backend:3000
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
