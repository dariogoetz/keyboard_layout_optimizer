const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin([
      'index.html',
      'codemirror.css',
      '../../layouts_webservice/static/vue-components.js',
      '../../layouts_webservice/static/app.css',
      '../../layouts_webservice/static/icons/github.svg',
    ])
  ],
  module: {
    rules: [
      {
        test: /\.txt$/i,
        use: 'raw-loader',
      },
      {
        test: /\.yml$/i,
        use: 'raw-loader',
      },
      {
        test: /worker\.js$/,
        loader: "worker-loader",
        options: {
          esModule: false,
        }
      },
      // {
      //   test: /\.js/i,
      //   use: 'raw-loader',
      // }
    ]
  },
  experiments: {
    asyncWebAssembly: true
  }
};
