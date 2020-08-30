const { next_prime } = require('../pkg/ssvm_wasm_rust_prime_lib.js');
console.log( next_prime("7") ); // -> 11