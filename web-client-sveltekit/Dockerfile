FROM node:21-alpine3.17 as build-stage

WORKDIR /usr/app
COPY web-client .

RUN npm i
RUN npm install dotenv
RUN npm run build

FROM node:21-alpine3.17 as run-stage

WORKDIR /usr/app

COPY --from=build-stage /usr/app/node_modules /usr/app/node_modules
COPY --from=build-stage /usr/app/package.json /usr/app/package.json
COPY --from=build-stage /usr/app/package-lock.json /usr/app/package-lock.json
COPY --from=build-stage /usr/app/build /usr/app/build
COPY --from=build-stage /usr/app/.env /usr/app/.env

CMD [ "node", "-r", "dotenv/config", "build" ]
EXPOSE 3000