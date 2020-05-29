import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    connected: false,
    coffeeWeight: 16,
    targetRatio: 2.5,
    preInfusion: 5.0,
    totalTime: 30.0,
    currentData: [{ x: 0, y: 0 }]
  },
  mutations: {
    setConnected(state, payload) {
      state.connected = payload.connected
    },
    setCoffeeWeight(state, payload) {
      state.coffeeWeight = Number.parseFloat(payload.weight)
    },
    setTargetRatio(state, payload) {
      state.targetRatio = Number.parseFloat(payload.ratio)
    },
    setPreInfusion(state, payload) {
      state.preInfusion = Number.parseFloat(payload.time)
    },
    setTotalTime(state, payload) {
      state.totalTime = Number.parseFloat(payload.time)
    }
  },
  actions: {
    connect({ commit }) {
      return new Promise((resolve) => {
        // do something async
        setTimeout(() => {
          commit({ type: 'setConnected', connected: true })
          resolve()
        }, 2000)
      })
    }
  },
  modules: {}
})
