const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
    mode: "production",
    entry: {
        index: "./js/index.ts"
    },
    output: {
        path: dist,
        filename: "[name].js"
    },
    devServer: {
        contentBase: dist,
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules|pkg/,
            },
        ],
    },
    plugins: [
        new CopyPlugin([
            path.resolve(__dirname, "static")
        ]),

        new WasmPackPlugin({
            crateDirectory: __dirname,
            extraArgs: "--out-name index"
        }),
    ]
};
