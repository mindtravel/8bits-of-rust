// store.js
// import { set } from 'core-js/core/dict';
// import { no } from 'element-plus/es/locale';
import { createStore } from 'vuex';
import createPersistedState from 'vuex-persistedstate';

export default createStore({
  state: {
    notes: [],
    patterns: [],
    songs: [],
    selectedNotes: new Set(),
    separatorPosition: 300,
    activeComposePage: "plugin",
    exportFormat: '',
    songName: '',
    estimated_space: 0,
    scrollX: 0,
    scrollY: 0,
    channels: [
      { id: 1, name: '音轨 1', patterns: [] },
      { id: 2, name: '音轨 2', patterns: [] },
      { id: 3, name: '音轨 3', patterns: [] },
      { id: 4, name: '音轨 4', patterns: [] },
      { id: 5, name: '音轨 5', patterns: [] }
    ]
  },
  mutations: {
    setSeparatorPosition(state, position) {
      state.separatorPosition = position;
    },
    setScrollPosition(state, { x, y }) {
      state.scrollX = x;
      state.scrollY = y;
    },
    addPatternToChannel(state, { channelId, pattern }) {
      const channel = state.channels.find(t => t.id === channelId);
      if (channel) {
        channel.patterns.push(pattern);
      }
    },
    updateChannelPattern(state, { channelId, patternId, pattern }) {
      const channel = state.channels.find(t => t.id === channelId);
      if (channel) {
        const index = channel.patterns.findIndex(p => p.id === patternId);
        channel.patterns.splice(index, 1, pattern);
      }
    },
    deleteChannelPattern(state, { channelId, patternId }) {
      const channel = state.channels.find(t => t.id === channelId);
      if (channel) {
        const index = channel.patterns.findIndex(p => p.id === patternId);
        channel.patterns.splice(index, 1);
      }
    },
    initNotes(state) {
      // console.log(typeof state.notes)
      // for (var i = 0; i < 88; ++i) {
      //   for (var j = 0; j < 16*8; ++j) {
      //     state.notes.grid[i][j] = 0;
      //   }
      // }
    // console.log("init notes", state.notes[11][21])
    },
    addNote(state, note) {
      state.notes.push(note)
      console.log("add notes")
      // for note in state.notes {
      //   console.log(note)
      // }
    },
    deleteNote(state, id) {
      state.notes = state.notes.filter(n => n.id !== id);
    },
    updateNoteLength(state, { id, length }) {
      const note = state.notes.find(n => n.id === id);
      if (note) note.length = length;
    },

    addPattern(state, pattern) {
      state.patterns.push(pattern)
      console.log("add patterns")
      // for pattern in state.patterns {
      //   console.log(pattern)
      // }
    },
    deletePattern(state, id) {
      state.patterns = state.patterns.filter(n => n.id !== id);
    },
    updatePatternLength(state, { id, length }) {
      const pattern = state.patterns.find(n => n.id === id);
      if (pattern) pattern.length = length;
    },



    updateSelection(state, { id, selected }) {
      selected ? state.selectedNotes.add(id) : state.selectedNotes.delete(id);
    },
    clearSelection(state) {
      state.selectedNotes.clear();
    },
    setExportFormat(state, format) {
      state.exportFormat = format;
    },

    addSong(state, song) {
      state.songs.push(song);
    },
    deleteSong(state, index) {
        state.songs.splice(index, 1);
    },
    setActiveComposePage(state, page) {
        state.activeComposePage = page;
    },
    setSongName(state, name) {
        state.songName = name;
    },
  },
  plugins: [createPersistedState()],
});