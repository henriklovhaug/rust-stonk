import React from 'react';
import { Route, Routes } from 'react-router-dom';

import MainPage from './MainPage';
import StonkPage from './StonkPage';

const Main = () => {
    return (
        <Routes>
            <Route path="/" element={<MainPage />} />
            <Route path="/stonk" element={<StonkPage timestamp={0} open={0} high={0} low={0} close={0} volume={0} adjclose={0} />} />
        </Routes>
    );
  }

export default Main;