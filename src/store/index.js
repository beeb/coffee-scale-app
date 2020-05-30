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
    btEnabled: false,
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
    setBtEnabled(state, payload) {
      state.btEnabled = payload.enabled
    },
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
      state.currentData.push({ x: payload.time, y: payload.weight })
    },
    clearCurrentData(state) {
      state.currentData = [{ x: 0, y: 0 }]
    },
    resetAppStatus(state) {
      state.connected = false
      state.currentWeight = 0.0
    }
  },
  actions: {
    checkBtStatus({ commit, state }) {
      return navigator.bluetooth.getAvailability().then((isBluetoothAvailable) => {
        commit({ type: 'setBtEnabled', enabled: isBluetoothAvailable })
        if (state.connected && !isBluetoothAvailable) {
          commit({ type: 'setConnected', connected: false })
        }
      })
    },
    connect({ commit }) {
      return navigator.bluetooth
        .requestDevice({ filters: [{ name: 'mpy-coffee' }, { services: [parseInt('0x1815')] }] })
        .then((device) => {
          let bluetoothDevice = device
          bluetoothDevice.addEventListener('gattserverdisconnected', () => {
            commit({ type: 'resetAppStatus' })
          })
          return device.gatt.connect()
        })
        .then((server) => {
          commit({ type: 'setConnected', connected: true })
          return server.getPrimaryService(parseInt('0x1815'))
        })
        .then((service) => {
          return service.getCharacteristic(parseInt('0x2A59')).then((characteristic) => {
            let weight = characteristic
            weight.startNotifications().then(() => {
              weight.addEventListener('characteristicvaluechanged', (ev) => {
                let bytes = ev.target.value
                let value = bytes.getInt16(0, false)
                console.log(value / 100)
                commit({ type: 'setCurrentWeight', weight: value / 100 })
              })
            })
          })
        })
    },
    readWeight({ commit, state }) {
      commit({ type: 'setCoffeeWeight', weight: state.currentWeight })
    }
  },
  modules: {}
})
