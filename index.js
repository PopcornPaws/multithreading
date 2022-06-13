const button = document.getElementById('render');
const canvas = document.getElementById('canvas');
const text = document.getElementById('text');
const concurrency = document.getElementById('concurrency');
const concurrencyAmt = document.getElementById('concurrency-amt');
const timing = document.getElementById('timing');
const timingVal = document.getElementById('timing-val');
const ctx = canvas.getContext('2d');

button.disabled = true;
concurrency.disabled = true;

// First up, but try to do feature detection to provide better error messages
function loadWasm() {
  let msg = 'This demo requires a current version of Firefox (e.g., 79.0)';
  if (typeof SharedArrayBuffer !== 'function') {
    alert('this browser does not have SharedArrayBuffer support enabled' + '\n\n' + msg);
    return
  }
  // Test for bulk memory operations with passive data segments
  //  (module (memory 1) (data passive ""))
  const buf = new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    0x05, 0x03, 0x01, 0x00, 0x01, 0x0b, 0x03, 0x01, 0x01, 0x00]);
  if (!WebAssembly.validate(buf)) {
    alert('this browser does not support passive wasm memory, demo does not work' + '\n\n' + msg);
    return
  }

  wasm_bindgen('./multi-wasm/wasmworkers_bg.wasm')
    .then(run)
    .catch(console.error);
}

loadWasm();

const { Text, WorkerPool } = wasm_bindgen;

function run() {
  // The maximal concurrency of our web worker pool is `hardwareConcurrency`,
  // so set that up here and this ideally is the only location we create web
  // workers.
  pool = new WorkerPool(navigator.hardwareConcurrency);

  // Configure various buttons and such.
  button.onclick = function() {
    button.disabled = true;
    console.time('render');
    process(text.value);
  };
  button.innerText = 'Render!';
  button.disabled = false;

  concurrency.oninput = function() {
    concurrencyAmt.innerText = 'Concurrency: ' + concurrency.value;
  };
  concurrency.min = 1;
  concurrency.step = 1;
  concurrency.max = navigator.hardwareConcurrency;
  concurrency.value = concurrency.max;
  concurrency.oninput();
  concurrency.disabled = false;
}

let rendering = null;
let start = null;
let interval = null;
let pool = null;

class State {
  constructor(wasm) {
    this.start = performance.now();
    this.wasm = wasm;
    this.running = true;
    this.counter = 1;

    this.interval = setInterval(() => this.updateTimer(true), 100);

    wasm
      .then(data => {
        this.updateTimer(false);
        console.log(data);
        this.stop();
      })
      .catch(console.error);
  }

  updateTimer(updateImage) {
    const dur = performance.now() - this.start;
    timingVal.innerText = `${dur}ms`;
    this.counter += 1;

    //if (updateImage && this.wasm && this.counter % 3 == 0)
    //  this.updateImage(this.wasm.imageSoFar());
  }

  //updateImage(data) {
  //  ctx.putImageData(data, 0, 0);
  //}

  stop() {
    if (!this.running)
      return;
    console.timeEnd('render');
    this.running = false;
    this.wasm = null;
    clearInterval(this.interval);
    button.disabled = false;
  }
}

function process(text) {
  if (rendering) {
    rendering.stop();
    rendering = null;
  }
  let txt = new Text(text);
  rendering = new State(txt.process(parseInt(concurrency.value), pool));
}
