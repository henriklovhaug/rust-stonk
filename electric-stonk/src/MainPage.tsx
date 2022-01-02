import React, { FC, useEffect } from 'react';
import logo from './logo.svg';
import './MainPage.css';
import { Stonk, ApiStonkNames } from './stonk';

const ws = new WebSocket('ws://localhost:8000/ws');

function MainPage() {
    const [stonk, setStonk] = React.useState<Stonk>({
        timestamp: 0,
        open: 0,
        high: 0,
        low: 0,
        close: 0,
        volume: 0,
        adjclose: 0
    });

    const [stonkNames, setStonkNames] = React.useState<ApiStonkNames[]>([]);

    ws.onmessage = function (event) {
        try {
            var my_object: ApiStonkNames[]  = JSON.parse(event.data);
            setStonkNames(my_object);
        } catch (error) {
            console.log(error);
        }
    }

    return (
        <div className="MainPage">
            <header className="MainPage-header">

                <input placeholder="Search for a stock" onChange={event => ws.send("search " + event.target.value)} />
                <ul className="stonk-list">
                    {stonkNames.map(stonkName => (
                        <li key={stonkName.stonk_name}>{stonkName.stonk_name}</li>
                    ))}
                </ul>

                <button onClick={() => connect()}>Send</button>
                {stonk.timestamp}
                <TestClass verdi = "test"/>
                {/* <img src={logo} className="App-logo" alt="logo" />
                <p>
                    Edit <code>src/App.tsx</code> and save to reload.
                </p> */}
            </header>
        </div>
    );
}

const TestClass: FC<TestProps>  = ({verdi}) => {
    return (
        <button>{verdi}</button>);
}

interface TestProps {
    verdi: string;
}

// ws.onopen = (event) => {
//   console.log('connected');
//   ws.send('hello');
// };

function connect() {
    ws.onclose = (event) => {
        console.log('disconnected');
    }
    //recconect
    ws.onopen = (event) => {
        console.log('reconnected');
    }
}


export default MainPage;
