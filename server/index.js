const express = require('express')
const app = express()
const port = 3000
var cors = require('cors')

app.use(cors())

app.get('/', (req, res) => {
  res.send('Hello World!')
})

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})

//GET DATA UNTUK DASHBOARD
app.get('/dashboard_data', (req, res) => {
  var data =
    {
      request_amount: 1412,
      ping: 57,
    }

  let condition = "server error not";

  if (condition == "server error") {
    let error = {
      error_description: "server currently error"
    }
    res.status(500).send(error)
  } else if (condition == "authentication failed") {
    let error = {
      error_description: "auth failed"
    }
    res.status(400).send(error)
  } else {
    res.status(200).send(data)
  }

})

