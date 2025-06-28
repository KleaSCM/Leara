/*
 * Leara AI Assistant - Main Entry Point
 * 
 * This file is the main entry point for the Leara AI Assistant frontend.
 * Initializes React and renders the main App component.
 * 
 * Copyright (c) 2024 Leara AI Assistant Contributors
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * Author: KleaSCM
 * Created: 2024-06-28
 * Last Modified: 2024-06-28
 * Version: 0.1.0
 * 
 * File: src/main.tsx
 * Purpose: Main React application entry point
 */

import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './styles/global.scss';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
); 