const HtmlWebpackPlugin = require('html-webpack-plugin');
const InlineChunkHtmlPlugin = require('react-dev-utils/InlineChunkHtmlPlugin');
const { VanillaExtractPlugin } = require('@vanilla-extract/webpack-plugin');

module.exports = {
	entry: './src/index.tsx',
	module: {
		rules: [
			{ test: /\.(js|ts|md)x?$/, exclude: /node_modules/, use: 'babel-loader' },
			{ test: /\.css$/i, use: ['style-loader', 'css-loader'] },
		]
	},
	plugins: [
		new HtmlWebpackPlugin({
			inject: 'body'
		}),
		new InlineChunkHtmlPlugin(HtmlWebpackPlugin, [/./]),
		new VanillaExtractPlugin(),
	],
	resolve: {
		extensions: ['.js', '.jsx', '.ts', '.tsx', '.mdx'],
	},
};
