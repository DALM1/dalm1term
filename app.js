import express from 'express';
import { exec } from 'child_process';

const app = express();
const port = 3000;

app.get('/execute', (req, res) => {
  const command = req.query.command;

  exec(command, { shell: true }, (error, stdout, stderr) => {
    if (error) {
      res.send(stderr);
    } else {
      res.send(stdout);
    }
  });
});

app.listen(port, () => {
  console.log(`Server running at http://localhost:${port}`);
});
