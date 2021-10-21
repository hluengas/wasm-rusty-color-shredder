const webpack = require('webpack');
const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: 'index.html'
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        })
    ],
    mode: 'development',
    ignoreWarnings: [
        (warning) =>
            warning.message ===
            "Critical dependency: the request of a dependency is an expression",
    ],
    experiments: {
        asyncWebAssembly: true
    }
};
