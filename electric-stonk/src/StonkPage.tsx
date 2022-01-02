import React, { FC, useEffect } from 'react';
import { Stonk, ApiStonkNames } from './stonk';
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
            

        </div>

    )

}

export default StonkPage;