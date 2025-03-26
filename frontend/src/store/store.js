// store.js
// import { set } from 'core-js/core/dict';
import { createStore } from 'vuex';
import createPersistedState from 'vuex-persistedstate';

export default createStore({
  state: {
    notes: [],
    selectedNotes: new Set(),
    separatorPosition: 300,
    songs: [],
    activeComposePage: "plugin",
    exportFormat: '',
    songName: '',
    estimated_space: 0,
    scrollX: 0,
    scrollY: 0,
    tracks: [
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
    addPatternToTrack(state, { trackId, pattern }) {
      const track = state.tracks.find(t => t.id === trackId);
      if (track) {
        track.patterns.push(pattern);
      }
    },
    updateTrackPattern(state, { trackId, patternId, pattern }) {
      const track = state.tracks.find(t => t.id === trackId);
      if (track) {
        const index = track.patterns.findIndex(p => p.id === patternId);
        track.patterns.splice(index, 1, pattern);
      }
    },
    deleteTrackPattern(state, { trackId, patternId }) {
      const track = state.tracks.find(t => t.id === trackId);
      if (track) {
        const index = track.patterns.findIndex(p => p.id === patternId);
        track.patterns.splice(index, 1);
      }
    },
    addNote(state, note) {
      state.notes.push(note);
    },
    deleteNote(state, id) {
      state.notes = state.notes.filter(n => n.id !== id);
    },
    updateNoteLength(state, { id, length }) {
      const note = state.notes.find(n => n.id === id);
      if (note) note.length = length;
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