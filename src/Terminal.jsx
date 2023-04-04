import React, { useState } from 'react';
import axios from 'axios';

const Terminal = () => {
  const [command, setCommand] = useState('');
  const [response, setResponse] = useState('');

  const executeCommand = () => {
    axios.get(`http://localhost:3000/execute?command=${command}`)
      .then(res => {
        setResponse(res.data);
      })
      .catch(err => {
        setResponse(err.message);
      });
  };

  const handleKeyDown = (event) => {
    if (event.key === 'Enter') {
      executeCommand();
    }
  };

  return (
    <div>
      <input type="text" value={command} onChange={e => setCommand(e.target.value)} onKeyDown={handleKeyDown} />
      <button onClick={executeCommand}>Execute</button>
      <pre>{response}</pre>
    </div>
  );
};

export default Terminal;
