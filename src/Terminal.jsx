import React, { useEffect, useRef, useState } from 'react';
import { initTauri } from '../tauri';

function Terminal() {
  const [output, setOutput] = useState('');
  const [input, setInput] = useState('');
  const [commandHistory, setCommandHistory] = useState([]);
  const [historyIndex, setHistoryIndex] = useState(-1);
  const inputEl = useRef(null);

  useEffect(() => {
    initTauri((output) => {
      setOutput((prevOutput) => prevOutput + output);
      inputEl.current.focus();
    });
  }, []);

  const handleInputChange = (event) => {
    setInput(event.target.value);
  };

  const handleKeyDown = (event) => {
    if (event.key === 'ArrowUp') {
      if (historyIndex === -1) {
        setHistoryIndex(commandHistory.length - 1);
        setInput(commandHistory[commandHistory.length - 1]);
      } else if (historyIndex > 0) {
        setHistoryIndex(historyIndex - 1);
        setInput(commandHistory[historyIndex - 1]);
      }
    } else if (event.key === 'ArrowDown') {
      if (historyIndex < commandHistory.length - 1) {
        setHistoryIndex(historyIndex + 1);
        setInput(commandHistory[historyIndex + 1]);
      } else {
        setHistoryIndex(-1);
        setInput('');
      }
    } else if (event.key === 'Enter') {
      if (input.trim()) {
        setCommandHistory((prevHistory) => [...prevHistory, input]);
        setHistoryIndex(-1);
        setInput('');
        initTauri(input + '\n');
      }
    }
  };

  return (
    <div className="terminal">
      <pre className="output">{output}</pre>
      <div className="input-container">
        {/* <span className="prompt">$</span> */}
        <input
          className="input"
          type="text"
          value={input}
          onChange={handleInputChange}
          onKeyDown={handleKeyDown}
          ref={inputEl}
          autoFocus
        />
      </div>
    </div>
  );
}

export default Terminal;
