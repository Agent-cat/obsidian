```docker
FROM node:18-alpine

WORKDIR /app

COPY package*.json ./

RUN npm install

RUN npm install -g serve

  

COPY . .

RUN npm run build

EXPOSE 5153

  

CMD ["serve", "-s", "dist", "-l", "5153"]
```

- Here we use **serve** to serve the **dist** folder which is usually done by nginx 