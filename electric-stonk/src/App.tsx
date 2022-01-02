import React from 'react';
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

// ws.onopen = (event) => {
//   console.log('connected');
//   ws.send('hello');
// };

export default App;
