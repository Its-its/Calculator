// const path = require('path');
// const HtmlWebpackPlugin = require('html-webpack-plugin');
// const webpack = require('webpack');
// const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

// module.exports = {
//     entry: './index.js',
//     output: {
//         path: path.resolve(__dirname, 'dist'),
//         filename: 'index.js',
//     },
//     plugins: [
//         new HtmlWebpackPlugin(),
//         new WasmPackPlugin({
//             crateDirectory: path.resolve(__dirname, ".")
//         }),
//         // Have this example work in Edge which doesn't ship `TextEncoder` or
//         // `TextDecoder` at this time.
//         new webpack.ProvidePlugin({
//           TextDecoder: ['text-encoding', 'TextDecoder'],
//           TextEncoder: ['text-encoding', 'TextEncoder']
//         })
//     ],
//     mode: 'development'
// };

const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "development",
  entry: {
    index: "./index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin([
      path.join(__dirname, "/static")
    ]),

    new WasmPackPlugin({
      crateDirectory: __dirname
    }),
  ]
};
