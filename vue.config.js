module.exports = {
  publicPath: '/',
  pwa: {
    name: 'Coffee Scale',
    themeColor: '#528078',
    msTileColor: '#ff6666',
    manifestOptions: {
      background_color: '#ffffff',
      display: 'standalone',
      orientation: 'landscape'
    },
    workboxPluginMode: 'InjectManifest',
    workboxOptions: {
      swSrc: 'src/service-worker.js'
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
