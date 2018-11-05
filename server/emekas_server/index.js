const express = require('express')
const bodyParser = require('body-parser');
const path = require('path')
const PORT = process.env.PORT || 8000

express()
  .use(express.static(path.join(__dirname, 'public')))
  .listen(PORT, () => console.log(`Listening on ${ PORT }`))