/**
 * Node.js test file for helloWeb example
 * We are not running this in CI currently, and Node.js is not a Broc dependency.
 * But if you happen to have it, you can run this.
 */

// Node doesn't have the fetch API
const fs = require("fs/promises");
global.fetch = (filename) =>
  fs.readFile(filename).then((buffer) => ({
    arrayBuffer() {
      return buffer;
    },
  }));

const { broc_web_platform_run } = require("./host");

broc_web_platform_run("./brocLovesWebAssembly.wasm", (string_from_broc) => {
  const expected = "Broc <3 Web Assembly!\n";
  if (string_from_broc !== expected) {
    console.error(`Expected "${expected}", but got "${string_from_broc}"`);
    process.exit(1);
  }
  console.log("OK");
});
