const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const srcDir = path.resolve(__dirname, "frontend", "src");
const distDir = path.resolve(__dirname, "frontend", "dist");

module.exports = {
  entry: {
    main: path.resolve(srcDir, "index.ts"),
  },
  output: {
    path: distDir,
    filename: "[name].js",
  },
  plugins: [new HtmlWebpackPlugin()],
};
