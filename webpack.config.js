const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  devServer: { contentBase: "./dist/" },
  plugins: [
    new CopyPlugin([{ from: "./src/index.html" }]),
    new WasmPackPlugin({ crateDirectory: __dirname })
  ]
};

// vim: set ts=2 sw=2 et:
