import { invoke } from '@tauri-apps/api/tauri';

// initialize Tauri API
invoke('tauri', {
  cmd: 'init',
});
