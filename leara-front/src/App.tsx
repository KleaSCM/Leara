/*
 * Leara AI Assistant - Main App Component
 * 
 * This component is the root of the Leara AI Assistant frontend application.
 * Contains the main layout and routing for the application.
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
 * File: src/App.tsx
 * Purpose: Main application component and layout
 */

import React, { useState, useEffect, useRef } from 'react';
import MemorySidebar from './components/MemorySidebar';
import TaskModal from './components/TaskModal';
import { 
  sendChatMessage, 
  addMemory, 
  addTask, 
  storeSessionContext,
  checkHealth 
} from './utils/api';
import './styles/App.scss';

interface Message {
  id: string;
  content: string;
  sender: 'user' | 'assistant';
  timestamp: Date;
}

const App: React.FC = () => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isSidebarOpen, setIsSidebarOpen] = useState(false);
  const [isTaskModalOpen, setIsTaskModalOpen] = useState(false);
  const [sessionId] = useState(() => `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`);
  const [backendStatus, setBackendStatus] = useState<'connected' | 'disconnected' | 'checking'>('checking');
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Check backend health on mount
  useEffect(() => {
    checkBackendHealth();
  }, []);

  // Auto-scroll to bottom when new messages arrive
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const checkBackendHealth = async () => {
    try {
      await checkHealth();
      setBackendStatus('connected');
    } catch (error) {
      setBackendStatus('disconnected');
      console.error('Backend health check failed:', error);
    }
  };

  const sendMessage = async () => {
    if (!inputValue.trim() || isLoading) return;

    const userMessage: Message = {
      id: Date.now().toString(),
      content: inputValue,
      sender: 'user',
      timestamp: new Date()
    };

    setMessages(prev => [...prev, userMessage]);
    const currentInput = inputValue;
    setInputValue('');
    setIsLoading(true);

    try {
      // Store session context
      await storeSessionContext({
        session_id: sessionId,
        context_key: 'last_user_message',
        context_value: currentInput
      });

      // Send chat message to backend
      const response = await sendChatMessage({
        message: currentInput,
        session_id: sessionId
      });

      const assistantMessage: Message = {
        id: (Date.now() + 1).toString(),
        content: response.message,
        sender: 'assistant',
        timestamp: new Date()
      };

      setMessages(prev => [...prev, assistantMessage]);

      // Check if the message contains task-related keywords and suggest task creation
      const taskKeywords = ['remind', 'todo', 'task', 'schedule', 'deadline', 'due'];
      const hasTaskKeywords = taskKeywords.some(keyword => 
        currentInput.toLowerCase().includes(keyword)
      );

      if (hasTaskKeywords) {
        const taskSuggestionMessage: Message = {
          id: (Date.now() + 2).toString(),
          content: "I detected this might be a task. Would you like me to create a task for you? You can click the 'Create Task' button above.",
          sender: 'assistant',
          timestamp: new Date()
        };
        setMessages(prev => [...prev, taskSuggestionMessage]);
      }

    } catch (error) {
      console.error('Error sending message:', error);
      const errorMessage: Message = {
        id: (Date.now() + 1).toString(),
        content: 'Sorry, I encountered an error while processing your message. Please try again.',
        sender: 'assistant',
        timestamp: new Date()
      };
      setMessages(prev => [...prev, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  const handleCreateTask = async (taskData: {
    title: string;
    description?: string;
    priority?: number;
    due_date?: string;
    tags?: string;
  }) => {
    try {
      await addTask({
        ...taskData,
        context: `Created from chat session: ${sessionId}`
      });
      
      const taskMessage: Message = {
        id: Date.now().toString(),
        content: `Task "${taskData.title}" has been created successfully!`,
        sender: 'assistant',
        timestamp: new Date()
      };
      setMessages(prev => [...prev, taskMessage]);
    } catch (error) {
      console.error('Error creating task:', error);
    }
  };

  const handleCreateMemory = async (key: string, value: string) => {
    try {
      await addMemory({
        key,
        value,
        category: 'chat',
        priority: 3,
        context: `Created from chat session: ${sessionId}`
      });
      
      const memoryMessage: Message = {
        id: Date.now().toString(),
        content: `Memory "${key}" has been stored successfully!`,
        sender: 'assistant',
        timestamp: new Date()
      };
      setMessages(prev => [...prev, memoryMessage]);
    } catch (error) {
      console.error('Error creating memory:', error);
    }
  };

  const handleTaskCreated = () => {
    // Refresh sidebar data if it's open
    if (isSidebarOpen) {
      // The sidebar will automatically refresh when it re-renders
    }
  };

  return (
    <div className="app">
      <header className="app-header">
        <div className="header-left">
          <h1>Leara AI Assistant</h1>
          <div className={`status-indicator ${backendStatus}`}>
            {backendStatus === 'connected' && '🟢 Connected'}
            {backendStatus === 'disconnected' && '🔴 Disconnected'}
            {backendStatus === 'checking' && '🟡 Checking...'}
          </div>
        </div>
        
        <div className="header-controls">
          <button 
            className="control-button"
            onClick={() => setIsSidebarOpen(!isSidebarOpen)}
            title="Memory & Tasks"
          >
            🧠
          </button>
          <button 
            className="control-button"
            onClick={() => setIsTaskModalOpen(true)}
            title="Create Task"
          >
            ➕
          </button>
          <button className="control-button" onClick={() => console.log('Minimize clicked')}>-</button>
          <button className="control-button" onClick={() => console.log('Maximize clicked')}>□</button>
          <button className="control-button" onClick={() => console.log('Close clicked')}>×</button>
        </div>
      </header>

      <main className={`app-main ${isSidebarOpen ? 'with-sidebar' : ''}`}>
        <div className="chat-container">
          <div className="messages">
            {messages.length === 0 && (
              <div className="welcome-message">
                <h2>Welcome to Leara AI Assistant!</h2>
                <p>I'm here to help you with tasks, remember important information, and assist with your daily activities.</p>
                <p>Try asking me to:</p>
                <ul>
                  <li>"Remind me to call John tomorrow"</li>
                  <li>"Remember that I prefer dark mode"</li>
                  <li>"What tasks do I have pending?"</li>
                  <li>"Search my memories for project ideas"</li>
                </ul>
              </div>
            )}
            
            {messages.map((message) => (
              <div key={message.id} className={`message ${message.sender}`}>
                <div className="message-content">{message.content}</div>
                <div className="message-timestamp">
                  {message.timestamp.toLocaleTimeString()}
                </div>
              </div>
            ))}
            
            {isLoading && (
              <div className="message assistant">
                <div className="message-content">
                  <div className="typing-indicator">
                    <span></span>
                    <span></span>
                    <span></span>
                  </div>
                </div>
              </div>
            )}
            <div ref={messagesEndRef} />
          </div>

          <div className="input-container">
            <textarea
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onKeyPress={handleKeyPress}
              placeholder="Type your message... (Press Enter to send, Shift+Enter for new line)"
              disabled={isLoading || backendStatus !== 'connected'}
            />
            <button 
              onClick={sendMessage} 
              disabled={isLoading || !inputValue.trim() || backendStatus !== 'connected'}
              className="send-button"
            >
              Send
            </button>
          </div>
        </div>
      </main>

      <MemorySidebar 
        isOpen={isSidebarOpen} 
        onClose={() => setIsSidebarOpen(false)} 
      />

      <TaskModal 
        isOpen={isTaskModalOpen}
        onClose={() => setIsTaskModalOpen(false)}
        onTaskCreated={handleTaskCreated}
      />
    </div>
  );
};

export default App; 