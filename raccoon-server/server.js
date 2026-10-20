const http = require('http');
const fs = require('fs'); 

const server = http.createServer((req, res) => {
const url = req.url;

  // The main purpose of this server is to serve JSON data files.
  // Serve the index.html file for the root URL is not requesting a data file.
  // Serve the pico.min.css file for the /pico.min.css URL
  // Serve the raccoon.ico file for the /favicon.ico URL
  
  if (url === '/') {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'text/html');
    fs.readFile('index.html', 'utf8', (err, data) => {
      if (err) {
        res.statusCode = 500;
        res.setHeader('Content-Type', 'text/html');
        res.end('<h1>Internal Server Error</h1>');
      } else {
        res.end(data);
      }
    });
  } else if (url === '/pico.min.css') {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'text/css');
    fs.readFile('pico.min.css', 'utf8', (err, data) => {
      if (err) {
        res.statusCode = 500;
        res.setHeader('Content-Type', 'text/html');
        res.end('<h1>Internal Server Error</h1>');
      } else {
        res.end(data);
      }
    }); 
  } else if (url === '/raccoon.ico') {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'image/x-icon');
    fs.readFile('raccoon.ico', (err, data) => {
      if (err) {
        res.statusCode = 500;
        res.setHeader('Content-Type', 'text/html');
        res.end('<h1>Internal Server Error</h1>');
      } else {
        res.end(data);
      }
    }); 
  } else {
    // Attempt to serve static files from the current directory
    const filePath = `.//pages//${url}`;
    fs.readFile(filePath, (err, data) => {
      if (err) {
        res.statusCode = 404;
        res.setHeader('Content-Type', 'text/html');
        res.end('<h1>404 Not Found</h1>');
      } else {
        // Determine the content type based on the file extension
        const ext = filePath.split('.').pop();
        let contentType = 'text/plain';
        if (ext === 'html') contentType = 'text/html';
        else if (ext === 'css') contentType = 'text/css';
        else if (ext === 'js') contentType = 'application/javascript';
        else if (ext === 'ico') contentType = 'image/x-icon';

        res.statusCode = 200;
        res.setHeader('Content-Type', contentType);
        res.end(data);
      }
    });
  }
});

server.listen(3000, () => {
  console.log('Server is listening on port 3000');
});