import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
 import { initTauri } from '../tauri';

function Terminal() {
  const [output, setOutput] = useState('');
  const [input, setInput] = useState('');

  useEffect(() => {
    const handleOutput = (event) => {
      setOutput((prevOutput) => prevOutput + event.payload + '\n');
    };

    invoke('start_shell').then(() => {
      initTauri({
        cmd: 'on',
        event: 'output',
        handler: handleOutput,
      });
    });

    return () => {
      initTauri({
        cmd: 'remove_handler',
        event: 'output',
      });
    };
  }, []);

  const handleSubmit = async (event) => {
    event.preventDefault();
    if (input.trim() !== '') {
      setOutput((prevOutput) => prevOutput + '> ' + input + '\n');
      const output = await invoke('run_command', { command: input });
      setOutput((prevOutput) => prevOutput + output + '\n');
      setInput('');
    }
  };

  const handleChange = (event) => {
    setInput(event.target.value);
  };

  return (
    <div>
      <pre>{output}</pre>
      <form onSubmit={handleSubmit}>
        <input type='text' value={input} onChange={handleChange} autoFocus />
        <button type='submit'>Send</button>
      </form>
    </div>
  );
}

export default Terminal;
