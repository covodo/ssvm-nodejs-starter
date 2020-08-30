const express = require('express');
const { next_prime } = require('../pkg/ssvm_wasm_rust_prime_lib.js');

const app = express();
const port = 3000;
app.use(express.static(__dirname + '/public'));
app.use(express.urlencoded({ extended: false }));

app.get('/', (req, res) => res.redirect("/app/node/public/index.html"));

app.post('/next', function (req, res) {
  let n = parseInt(req.body.n).toString();
  res.send(next_prime(n));
});

app.listen(port, () => console.log(`Listening at http://localhost:${port}`));