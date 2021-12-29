import React, { FunctionComponent, useEffect } from 'react';
import logo from './logo.svg';
import './App.css';
import { Stonk } from './stonk';

const ws = new WebSocket('ws://localhost:8000/ws');

function App() {
    const [stonk, setStonk] = React.useState<Stonk>({
        timestamp: 0,
        open: 0,
        high: 0,
        low: 0,
        close: 0,
        volume: 0,
        adjclose: 0
    });

    ws.onmessage = function (event) {
        //console.log("STONKUS " + event.data);
        var json: Stonk[] = JSON.parse(event.data);
        setStonk(json[0]);
    }

    return (
        <div className="App">
            <header className="App-header">

                <button onClick={() => ws.send('stonk TSLA')}>Send</button>
                {stonk.timestamp}
                <TestClass verdi = "test"/>
                <img src={logo} className="App-logo" alt="logo" />
                <p>
                    Edit <code>src/App.tsx</code> and save to reload.
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

const TestClass: FunctionComponent<TestProps>  = ({verdi}) => {
    return (
        <button>{verdi}</button>);
}

interface TestProps {
    verdi: string;
}

ws.onopen = (event) => {
  console.log('connected');
  ws.send('hello');
};


export default App;
