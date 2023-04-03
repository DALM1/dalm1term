import { invoke } from '@tauri-apps/api/tauri';

export function initTauri(callback) {
  invoke('tauri', {
    cmd: 'init',
  }).then((_) => {
    invoke('tauri', {
      cmd: 'listen',
      callback: (message) => {
        callback(message);
      },
    });
  });
}
