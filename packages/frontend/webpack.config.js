const HtmlWebpackPlugin = require('html-webpack-plugin');
const InlineChunkHtmlPlugin = require('react-dev-utils/InlineChunkHtmlPlugin');

module.exports = {
	module: {
		rules: [
			{ test: /\.jsx?$/, use: 'babel-loader' }
		]
	},
	plugins: [
		new HtmlWebpackPlugin({
			inject: true
		}),
		new InlineChunkHtmlPlugin(HtmlWebpackPlugin, [/./]),
	]
};
