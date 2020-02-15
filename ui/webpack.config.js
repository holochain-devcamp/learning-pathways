const path = require('path');
const webpack = require('webpack');
const { createDefaultConfig } = require('@open-wc/building-webpack');

// if you need to support IE11 use "modern-and-legacy-config" instead.
// const { createCompatibilityConfig } = require('@open-wc/building-webpack');
// module.exports = createCompatibilityConfig({
//   input: path.resolve(__dirname, './index.html'),
// });

const config = createDefaultConfig({
  input: path.resolve(__dirname, './index.html'),
  mode: 'development'
});

module.exports = {
  ...config,
  plugins: [
    ...config.plugins,
    new webpack.EnvironmentPlugin({
      WS_INTERFACE: process.env.WS_INTERFACE,
      USERNAME: process.env.USERNAME
    })
  ]
};
