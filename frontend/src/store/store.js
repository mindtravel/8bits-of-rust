// store.js
import { createStore } from 'vuex';
import createPersistedState from 'vuex-persistedstate';

export default createStore({
  state: {
    songs: [],
    compose: "synth",
  },
  mutations: {
    addSong(state, song) {
      state.songs.push(song);
    },
    deleteSong(state, index) {
        state.songs.splice(index, 1);
    },
    changeCompose(state, compose) {
        state.compose = compose;
    },
  },
  plugins: [createPersistedState()] // ƒ¨»œ π”√localStorage
});