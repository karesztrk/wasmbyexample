// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "../pkg/fibonacci.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const helloWorld = await init("../pkg/fibonacci_bg.wasm");

  // Call the Add function export from wasm, save the result
  const addResult = helloWorld.fibonacci(10);

  // Set the result onto the body
  document.body.textContent = `Hello World! Fibonacci: ${addResult}`;
};
runWasm();
