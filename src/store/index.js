import Vue from 'vue'
import Vuex from 'vuex'
import VuexPersist from 'vuex-persist'

Vue.use(Vuex)

let keepMutations = ['setCoffeeWeight', 'setTargetRatio', 'setPreInfusion', 'setTotalTime']

const vuexLocalStorage = new VuexPersist({
  key: 'vuex',
  storage: window.localStorage,
  reducer: (state) => ({
    coffeeWeight: state.coffeeWeight,
    targetRatio: state.targetRatio,
    preInfusion: state.preInfusion,
    totalTime: state.totalTime
  }),
  filter: (mutation) => keepMutations.indexOf(mutation.type) > -1
})

export default new Vuex.Store({
  plugins: [vuexLocalStorage.plugin],
  state: {
    connected: false,
    coffeeWeight: 16,
    targetRatio: 2.5,
    preInfusion: 5.0,
    totalTime: 30.0,
    currentWeight: 30.0,
    currentData: [{ x: 0, y: 0 }]
  },
  getters: {
    targetWeight: (state) => {
      return state.coffeeWeight * state.targetRatio
    }
  },
  mutations: {
    setConnected(state, payload) {
      state.connected = payload.connected
    },
    setCoffeeWeight(state, payload) {
      state.coffeeWeight = payload.weight
    },
    setTargetRatio(state, payload) {
      state.targetRatio = payload.ratio
    },
    setPreInfusion(state, payload) {
      state.preInfusion = payload.time
    },
    setTotalTime(state, payload) {
      let val = Math.max(state.preInfusion, payload.time)
      state.totalTime = val
    },
    setCurrentWeight(state, payload) {
      state.currentWeight = payload.weight
    },
    addDataPoint(state, payload) {
      state.currentData.push({ x: payload.x, y: payload.y })
    },
    clearCurrentData(state) {
      state.currentData = [{ x: 0, y: 0 }]
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
