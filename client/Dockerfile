FROM node:21-alpine

WORKDIR /app
COPY . /app

RUN npm install
RUN npm run build

EXPOSE 4000

ENV PORT="4000"

CMD ["node", "build"]
