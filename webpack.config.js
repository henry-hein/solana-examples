const CopyPlugin = require('copy-webpack-plugin');
const path = require('path');
const webpack = require('webpack');
require('dotenv').config();

// Use a function to configure based on the environment
module.exports = (env, argv) => {
  const isDevelopment = argv.mode === 'development';

  return {
    // 1. Differentiate build modes
    mode: argv.mode,
    
    // 2. Use source maps only in development for easier debugging
    devtool: isDevelopment ? 'eval-source-map' : false, 

    entry: {
      'program_1': './front/src/program_1.ts',
      'program_2': './front/src/program_2.ts',
      'program_3': './front/src/program_3.ts',
      'program_4': './front/src/program_4.ts',
    },
    output: {
      filename: '[name].js',
      path: path.resolve(__dirname, 'dist'),
      // Clean the dist directory on every build
      clean: true, 
    },
    module: {
      rules: [
        {
          test: /\.(ts|tsx)$/,
          use: 'babel-loader',
          exclude: /node_modules/,
        },
      ],
    },
    resolve: {
      extensions: ['.tsx', '.ts', '.js'],
      // Fallback for Node-specific modules needed by Solana/Wert libs in the browser
      fallback: {
        "buffer": require.resolve("buffer/"),
      }
    },
    plugins: [
      new CopyPlugin({
        patterns: [
          // This copies your static HTML file into the output directory
          { from: 'static', to: path.resolve(__dirname, 'dist') }, 
        ],
      }),
      new webpack.DefinePlugin({
        // Define environment variables so they can be used inside your JS/TS code
        'process.env.API_KEY': JSON.stringify(process.env.API_KEY),
        'process.env.PARTNER_ID': JSON.stringify(process.env.PARTNER_ID),
      }),
      // Provide the Buffer global for older libraries
      new webpack.ProvidePlugin({
        Buffer: ['buffer', 'Buffer'],
      }),
    ],
    devServer: {
      static: {
        // Serve files from the dist directory
        directory: path.join(__dirname, 'dist'),
      },
      client: {
        overlay: false,
      },
      port: 8765,
    },
  };
};