FROM node

WORKDIR /gateway

COPY --chown=node:node . .

RUN npm install && npm cache clean --force --loglevel=error

CMD [ "npm", "start"]
