FROM node:24-alpine AS deps

WORKDIR /app

COPY package.json package-lock.json ./

RUN npm ci --omit=dev


FROM node:26-alpine AS builder

WORKDIR /app

COPY package.json package-lock.json ./
RUN npm ci

COPY . .

RUN npm run build


FROM node:26-alpine AS runner

WORKDIR /app

ENV NODE_ENV=production

RUN addgroup -S -g 1001 nextjs \
    && adduser -S -u 1001 -G nextjs nextjs

COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static
COPY --from=builder /app/public ./public

RUN mkdir -p /app/.next/cache \
    && chown -R nextjs:nextjs /app/.next

USER nextjs

EXPOSE 3000

CMD ["node", "server.js"]
