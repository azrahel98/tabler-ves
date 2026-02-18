# Etapa 1: Construcción
FROM node:22-alpine AS build
WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm install
COPY . .
RUN npm run build

# Etapa 2: Servidor
FROM nginx:alpine
# Se copia el contenido de dist generado en la etapa anterior
COPY --from=build /app/dist /usr/share/nginx/html
# Se copia tu config a conf.d para no romper el nginx.conf global
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 4001
CMD ["nginx", "-g", "daemon off;"]