import { useState } from 'react';

function Terminal() {
  const [output, setOutput] = useState('');
  const [command, setCommand] = useState('');

  const handleInputChange = (event) => {
    setCommand(event.target.value);
  };

  const handleKeyDown = (event) => {
    if (event.key === 'Enter') {
      if (command.trim()) {
        setOutput(output + '$ ' + command + '\n');
        setCommand('');
        executeCommand(command);
      }
    }
  };

  const executeCommand = async (command) => {
    try {
      const response = await fetch(`https://api.commandline.run/cmd/${encodeURIComponent(command)}`);
      const text = await response.text();
      setOutput(output + text + '\n');
    } catch (error) {
      setOutput(output + 'Error: ' + error.message + '\n');
    }
  };

  return (
    <div>
      <pre>{output}</pre>
      <input type="text" value={command} onChange={handleInputChange} onKeyDown={handleKeyDown} autoFocus />
    </div>
  );
}

export default Terminal;
