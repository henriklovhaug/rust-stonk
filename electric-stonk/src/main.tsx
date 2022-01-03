import React from 'react';
import { Route, Routes } from 'react-router-dom';

import MainPage from './MainPage';
import StonkPage from './StonkPage';

const Main = () => {
    return (
        <Routes>
            <Route path="/" element={<MainPage />} />
            <Route path="/stonk/:id" element={<StonkPage />} />
        </Routes>
    );
}

export default Main;