// index.js
import init, { Aquarium } from './pkg/aquarium.js';

async function run() {
    await init(); // Load the WASM module from the 'aquarium' crate

    // Here we will initialize the Aquarium background
}

run();