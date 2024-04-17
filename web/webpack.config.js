const path = require('path');
const HTMLWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  mode: 'development',
  target: ['web', 'es5'],
  entry: {
    main: './src/index.tsx',
  },
  output: {
    filename: '[name].bundle.js',
    path: path.resolve(__dirname, 'dist')
  },
  module: {
    rules: [
      {
        test: /\.(ts|tsx)$/,
        use: 'babel-loader'
      },
      {
        test: /\.(jpg|png)/,
        type: 'asset/resource'
      },
    ]
  },
  plugins: [
    new HTMLWebpackPlugin({
      title: '首页',
      filename: 'index.html',
      template: './src/index.ejs',
      chunks: ['main']
    })
  ],
  experiments: {
    asyncWebAssembly: true
  }
};