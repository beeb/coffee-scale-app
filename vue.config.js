module.exports = {
  publicPath: process.env.NODE_ENV === 'production' ? '/coffee/' : '/',
  pwa: {
    name: 'Coffee Scale',
    themeColor: '#7C6D68',
    msTileColor: '#DD545C',
    manifestOptions: {
      background_color: '#6C5662',
      display: 'standalone',
      orientation: 'landscape'
    }
  },
  devServer: {
    https: true
  },
  pages: {
    index: {
      entry: 'src/main.js',
      title: 'Coffee Scale'
    }
  },
  lintOnSave: false
}
