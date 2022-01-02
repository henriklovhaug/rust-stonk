import React from 'react';
import { Link, useParams } from 'react-router-dom';
import { ApiStonkNames, Stonk } from './stonk';
import "./StonkPage.css";

var ws = new WebSocket('ws://localhost:8000/ws');

function StonkPage() {

    const myStonkName = useParams<{ id: string }>().id;
    ws.send("stonk " + myStonkName);
    const [stonk, setStonk] = React.useState<Stonk>({
            timestamp: 0,
            open: 0,
            high: 0,
            low: 0,
            close: 0,
            volume: 0,
            adjclose: 0
        });

    ws.onclose = function (event) {
        //Wait a second and try to reconnect
        setTimeout(() => {
            ws = new WebSocket('ws://localhost:8000/ws');
        }, 1000);
    }

    ws.onmessage = function (event) {
        try {
            var my_object: Stonk[]  = JSON.parse(event.data);
            setStonk(my_object[0]);
        } catch (error) {
            console.log(error);
        }
    }

    //Useeffect cleanup
    React.useEffect(() => {
        return () => {
            ws.close();
        }
    }, []);

    return (
        <div className="stonk-page">
            <header className='stonk-page-header'>
                {stonk.open}
                <Link to={'/'}>
                    <button>Back</button>
                </Link>
            </header>

        </div>

    )

}

export default StonkPage;