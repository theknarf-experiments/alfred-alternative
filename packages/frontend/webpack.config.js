const HtmlWebpackPlugin = require('html-webpack-plugin');
const InlineChunkHtmlPlugin = require('react-dev-utils/InlineChunkHtmlPlugin');

module.exports = {
	entry: './src/index.tsx',
	module: {
		rules: [
			{ test: /\.(js|ts|md)x?$/, exclude: /node_modules/, use: 'babel-loader' }
		]
	},
	plugins: [
		new HtmlWebpackPlugin({
			inject: 'body'
		}),
		new InlineChunkHtmlPlugin(HtmlWebpackPlugin, [/./]),
	],
	resolve: {
		extensions: ['.js', '.jsx', '.ts', '.tsx', '.mdx'],
	},
};
