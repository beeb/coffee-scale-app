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
    btServer: null,
    batteryLevel: null,
    coffeeWeight: 16,
    targetRatio: 2.5,
    preInfusion: 5.0,
    totalTime: 30.0,
    recording: false,
    startTimeMs: 0,
    currentWeight: 0.0,
    currentData: [{ x: 0, y: 0 }]
  },
  getters: {
    targetWeight: (state) => {
      return state.coffeeWeight * state.targetRatio
    },
    elapsedTime: (state) => {
      if (state.startTimeMs === 0) {
        return 0
      }
      let now = new Date().getTime()
      return (now - state.startTimeMs) / 1000
    }
  },
  mutations: {
    setBtEnabled(state, payload) {
      state.btEnabled = payload.enabled
    },
    setConnected(state, payload) {
      state.connected = payload.connected
    },
    setBtServer(state, payload) {
      state.btServer = payload.server
    },
    setBatteryLevel(state, payload) {
      state.batteryLevel = payload.battery
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
    setRecording(state, payload) {
      state.recording = payload.recording
    },
    setCurrentWeight(state, payload) {
      state.currentWeight = payload.weight
      let now = new Date().getTime()
      if (state.recording && state.startTimeMs === 0 && payload.weight > 0.5) {
        state.startTimeMs = now - state.preInfusion * 1000
      }
      if (state.recording && state.startTimeMs > 0 && payload.weight < -0.1) {
        state.recording = false
        state.startTimeMs = 0
      }
      if (state.recording && state.startTimeMs > 0) {
        let elapsed = (now - state.startTimeMs) / 1000
        state.currentData.push({ x: elapsed, y: payload.weight })
      }
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
      state.batteryLevel = null
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
        .requestDevice({ filters: [{ name: 'mpy-coffee' }, { services: [parseInt('0x1815'), parseInt('0x180F')] }] })
        .then((device) => {
          let bluetoothDevice = device
          bluetoothDevice.addEventListener('gattserverdisconnected', () => {
            commit({ type: 'resetAppStatus' })
          })
          return device.gatt.connect()
        })
        .then((server) => {
          commit({ type: 'setConnected', connected: true })
          commit({ type: 'setBtServer', server: server })
          return server.getPrimaryService(parseInt('0x1815'))
        })
        .then((service) => {
          return service.getCharacteristic(parseInt('0x2A59'))
        })
        .then((characteristic) => {
          let weight = characteristic
          return weight.startNotifications().then(() => {
            weight.addEventListener('characteristicvaluechanged', (ev) => {
              let bytes = ev.target.value
              let value = bytes.getInt16(0, false)
              commit({ type: 'setCurrentWeight', weight: value / 100 })
            })
          })
        })
    },
    readWeight({ commit, state }) {
      commit({ type: 'setCoffeeWeight', weight: state.currentWeight })
    },
    startRecording({ commit }) {
      commit({ type: 'clearCurrentData' })
      commit({ type: 'setRecording', recording: true })
    },
    getBatteryLevel({ commit, state }) {
      if (state.btServer === null) {
        return
      }
      return state.btServer
        .getPrimaryService(parseInt('0x180F'))
        .then((service) => {
          return service.getCharacteristic(parseInt('0x2A19'))
        })
        .then((characteristic) => {
          return characteristic.readValue()
        })
        .then((value) => {
          let batteryLevel = value.getUint8(0)
          console.log(batteryLevel)
          commit({ type: 'setBatteryLevel', battery: batteryLevel })
        })
    }
  },
  modules: {}
})
