module.exports = {
  pages: {
    index: {
      entry: 'src/main.js',
      title: 'MP4 Inspector',
    }
  },
  publicPath: process.env.NODE_ENV === 'production'
    ? '/mp4-inspector/'
    : '/',
};
