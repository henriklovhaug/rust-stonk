import React from 'react';
import { render, screen } from '@testing-library/react';
import App from './App';
import { MemoryRouter } from 'react-router-dom';

test('Find button', () => {
  render(<MemoryRouter>
    <App />
  </MemoryRouter>);
  const button = screen.getByText(/Send/i);
  expect(button).toBeInTheDocument();
});
