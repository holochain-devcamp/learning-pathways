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
  mode: process.env.PRODUCTION ? 'production' : 'development'
});

module.exports = {
  ...config,
  plugins: [
    ...config.plugins,
    new webpack.EnvironmentPlugin({
      WS_INTERFACE: process.env.WS_INTERFACE,
      PRODUCTION: process.env.PRODUCTION,
      USERNAME: process.env.USERNAME
    })
  ]
};
