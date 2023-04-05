import React, { useState, useEffect, useRef } from 'react';
import axios from 'axios';

const Terminal = () => {
  const [command, setCommand] = useState('');
  const [response, setResponse] = useState('');
  const inputRef = useRef(null);

  const executeCommand = () => {
    axios.get(`http://localhost:3000/execute?command=${command}`)
      .then(res => {
        setResponse(res.data);
        setCommand('');
        inputRef.current.focus();
      })
      .catch(err => {
        setResponse(err.message);
        setCommand('');
        inputRef.current.focus();
      });
  };

  const handleKeyDown = (event) => {
    if (event.key === 'Enter') {
      executeCommand();
    }
  };

  useEffect(() => {
    inputRef.current.focus();
  }, []);

  return (
    <div>
      <input type="text" value={command} onChange={e => setCommand(e.target.value)} onKeyDown={handleKeyDown} ref={inputRef} />
      <button onClick={executeCommand}>Execute</button>
      <pre>{response}</pre>
    </div>
  );
};

export default Terminal;
