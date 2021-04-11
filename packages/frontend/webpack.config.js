const HtmlWebpackPlugin = require('html-webpack-plugin');
const InlineChunkHtmlPlugin = require('react-dev-utils/InlineChunkHtmlPlugin');

module.exports = {
	module: {
		rules: [
			{ test: /\.jsx?$/, exclude: /node_modules/, use: 'babel-loader' }
		]
	},
	plugins: [
		new HtmlWebpackPlugin({
			inject: 'body'
		}),
		new InlineChunkHtmlPlugin(HtmlWebpackPlugin, [/./]),
	]
};
