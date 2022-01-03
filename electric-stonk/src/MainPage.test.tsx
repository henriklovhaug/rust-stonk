import React from 'react';
import { render, screen } from '@testing-library/react';
import MainPage from './MainPage';
import { MemoryRouter } from 'react-router-dom';

test('Find some stocks', async () => {
    render(<MemoryRouter>
        <MainPage />
        </MemoryRouter>);
    //Write Apple in input
    const input: HTMLInputElement = screen.getByPlaceholderText(/Search for a stock/i);
    input.value = "Apple";
    //input.dispatchEvent(new Event('Apple'));
    //Assert AAPL is in the list

    //This test can't connect to the server
    // const button = screen.getByText(/AAPL/i);
    // expect(button).toBeInTheDocument();
});
