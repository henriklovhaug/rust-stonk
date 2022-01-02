import React, { FC, useEffect } from 'react';
import logo from './logo.svg';
import './App.css';
import Main from './main';

function App() {


    return (
        <div className="App">
            <header className="App-header">

                <div className="router">
                    <Main />
                </div>
            </header>
        </div>
    );
}

const TestClass: FC<TestProps> = ({ verdi }) => {
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


export default App;
