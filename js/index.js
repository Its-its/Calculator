const path = require('path');

const express = require('express');


const app = express();

app.use(express.static(path.join(__dirname, '../dist')));
app.use('/static', express.static(path.join(__dirname, '../public')));


app.get('/', (_, res) => res.sendFile(path.join(__dirname, '../static/index.html')));


app.listen('8080', () => console.log('Started'));