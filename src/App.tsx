import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import './App.css';
import init, { add } from "rust-wasm-lib";

function App() {
  const [count, setCount] = useState(0);
  useEffect(() => {
    init()
  }, [])

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          <button onClick={() => setCount(add(1,count))}>
          Click me
          </button>
          <p>2^{count} = {Math.pow(2, count)}</p>
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
