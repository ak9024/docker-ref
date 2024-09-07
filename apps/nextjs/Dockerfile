FROM node:20-alpine AS deps

COPY package*.json ./

RUN npm ci

FROM node:20-alpine AS builder

COPY . .
COPY --from=deps /node_modules ./node_modules
RUN npm run build

FROM node:20-alpine AS runner

COPY --from=builder ./node_modules/ ./node_modules
COPY --from=builder /.next/standalone ./
# https://nextjs.org/docs/pages/api-reference/next-config-js/assetPrefix
# COPY --from=builder /public ./public
# COPY --from=builder /.next/static ./.next/static

CMD node server.js
