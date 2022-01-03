import React from 'react';
import { render, screen } from '@testing-library/react';
import App from './App';
import { MemoryRouter } from 'react-router-dom';

test('renders learn react link', () => {
  render(<MemoryRouter>
    <App />
  </MemoryRouter>);
  const button = screen.getByText(/Send/i);
  expect(button).toBeInTheDocument();
});
