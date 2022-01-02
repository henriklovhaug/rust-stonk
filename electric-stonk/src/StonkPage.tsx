import React from 'react';
import { Link } from 'react-router-dom';
import { Stonk } from './stonk';
import "./StonkPage.css";

var ws = new WebSocket('ws://localhost:8000/ws');

function StonkPage(stonk:Stonk) {
    ws.onclose = function (event) {
        //Wait a second and try to reconnect
        setTimeout(() => {
            ws = new WebSocket('ws://localhost:8000/ws');
        }, 1000);
    }

    return (
        <div className="stonk-page">
            <header className='stonk-page-header'>
                <Link to={'/'}>
                    <button>Back</button>
                </Link>
            </header>

        </div>

    )

}

export default StonkPage;