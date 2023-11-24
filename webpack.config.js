const path = require("path");

const dist = path.resolve(__dirname, "dist");

module.exports = {
    mode: "production",
    entry: {
        index: "./js/index.ts"
    },
    output: {
        path: dist,
        filename: "[name].js",
        libraryTarget: 'commonjs-module'
    },
    devServer: {
        contentBase: dist,
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
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
};
