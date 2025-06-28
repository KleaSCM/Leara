/*
 * Leara AI Assistant - Memory Sidebar Component
 * 
 * This component provides a sidebar for viewing and managing memories,
 * tasks, and searching through stored information.
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
 * File: src/components/MemorySidebar.tsx
 * Purpose: Memory and task management sidebar
 */

import React, { useState, useEffect } from 'react';
import { 
  Memory, Task, MemoryResponse, TaskResponse, 
  getMemories, getTasks, searchMemories, getMemorySummary,
  updateTaskStatus 
} from '../utils/api';
import '../styles/MemorySidebar.scss';

type TabType = 'memories' | 'tasks' | 'search' | 'summary';

interface MemorySidebarProps {
  isOpen: boolean;
  onClose: () => void;
}

const MemorySidebar: React.FC<MemorySidebarProps> = ({ isOpen, onClose }) => {
  const [activeTab, setActiveTab] = useState<TabType>('memories');
  const [memories, setMemories] = useState<Memory[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [searchResults, setSearchResults] = useState<Memory[]>([]);
  const [summary, setSummary] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  // Load data based on active tab
  useEffect(() => {
    if (!isOpen) return;

    const loadData = async () => {
      setLoading(true);
      setError('');
      
      try {
        switch (activeTab) {
          case 'memories':
            const memoryResponse = await getMemories();
            setMemories(memoryResponse.memories);
            break;
          case 'tasks':
            const taskResponse = await getTasks();
            setTasks(taskResponse.tasks);
            break;
          case 'summary':
            const summaryResponse = await getMemorySummary();
            setSummary(summaryResponse.summary);
            break;
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load data');
      } finally {
        setLoading(false);
      }
    };

    loadData();
  }, [activeTab, isOpen]);

  const handleSearch = async () => {
    if (!searchQuery.trim()) return;

    setLoading(true);
    setError('');
    
    try {
      const response = await searchMemories(searchQuery);
      setSearchResults(response.memories);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to search memories');
    } finally {
      setLoading(false);
    }
  };

  const handleTaskStatusUpdate = async (taskId: number, newStatus: string) => {
    try {
      await updateTaskStatus(taskId, newStatus);
      // Refresh tasks
      const taskResponse = await getTasks();
      setTasks(taskResponse.tasks);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update task status');
    }
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString();
  };

  const getPriorityColor = (priority: number) => {
    switch (priority) {
      case 5: return 'priority-high';
      case 4: return 'priority-medium-high';
      case 3: return 'priority-medium';
      case 2: return 'priority-low';
      default: return 'priority-medium';
    }
  };

  const getStatusColor = (status: string) => {
    switch (status.toLowerCase()) {
      case 'completed': return 'status-completed';
      case 'in_progress': return 'status-in-progress';
      case 'pending': return 'status-pending';
      default: return 'status-pending';
    }
  };

  if (!isOpen) return null;

  return (
    <div className="memory-sidebar">
      <div className="sidebar-header">
        <h2>Memory & Tasks</h2>
        <button className="close-button" onClick={onClose}>×</button>
      </div>

      <div className="sidebar-tabs">
        <button 
          className={`tab ${activeTab === 'memories' ? 'active' : ''}`}
          onClick={() => setActiveTab('memories')}
        >
          Memories
        </button>
        <button 
          className={`tab ${activeTab === 'tasks' ? 'active' : ''}`}
          onClick={() => setActiveTab('tasks')}
        >
          Tasks
        </button>
        <button 
          className={`tab ${activeTab === 'search' ? 'active' : ''}`}
          onClick={() => setActiveTab('search')}
        >
          Search
        </button>
        <button 
          className={`tab ${activeTab === 'summary' ? 'active' : ''}`}
          onClick={() => setActiveTab('summary')}
        >
          Summary
        </button>
      </div>

      <div className="sidebar-content">
        {error && <div className="error-message">{error}</div>}
        
        {loading && <div className="loading">Loading...</div>}

        {activeTab === 'memories' && (
          <div className="memories-tab">
            <h3>Recent Memories ({memories.length})</h3>
            {memories.length === 0 ? (
              <p className="empty-state">No memories stored yet.</p>
            ) : (
              <div className="memories-list">
                {memories.map((memory) => (
                  <div key={memory.id} className="memory-item">
                    <div className="memory-header">
                      <span className="memory-key">{memory.key}</span>
                      <span className={`priority-badge ${getPriorityColor(memory.priority)}`}>
                        P{memory.priority}
                      </span>
                    </div>
                    <div className="memory-value">{memory.value}</div>
                    <div className="memory-meta">
                      <span className="category">{memory.category}</span>
                      <span className="date">{formatDate(memory.created_at)}</span>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}

        {activeTab === 'tasks' && (
          <div className="tasks-tab">
            <h3>Tasks ({tasks.length})</h3>
            {tasks.length === 0 ? (
              <p className="empty-state">No tasks found.</p>
            ) : (
              <div className="tasks-list">
                {tasks.map((task) => (
                  <div key={task.id} className="task-item">
                    <div className="task-header">
                      <span className="task-title">{task.title}</span>
                      <span className={`priority-badge ${getPriorityColor(task.priority)}`}>
                        P{task.priority}
                      </span>
                    </div>
                    {task.description && (
                      <div className="task-description">{task.description}</div>
                    )}
                    <div className="task-meta">
                      <span className={`status-badge ${getStatusColor(task.status)}`}>
                        {task.status}
                      </span>
                      {task.due_date && (
                        <span className="due-date">Due: {formatDate(task.due_date)}</span>
                      )}
                    </div>
                    <div className="task-actions">
                      {task.status === 'pending' && (
                        <button 
                          onClick={() => handleTaskStatusUpdate(task.id, 'in_progress')}
                          className="action-button"
                        >
                          Start
                        </button>
                      )}
                      {task.status === 'in_progress' && (
                        <button 
                          onClick={() => handleTaskStatusUpdate(task.id, 'completed')}
                          className="action-button"
                        >
                          Complete
                        </button>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}

        {activeTab === 'search' && (
          <div className="search-tab">
            <h3>Search Memories</h3>
            <div className="search-input">
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search for memories..."
                onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
              />
              <button onClick={handleSearch} disabled={!searchQuery.trim()}>
                Search
              </button>
            </div>
            
            {searchResults.length > 0 && (
              <div className="search-results">
                <h4>Search Results ({searchResults.length})</h4>
                {searchResults.map((memory) => (
                  <div key={memory.id} className="memory-item">
                    <div className="memory-header">
                      <span className="memory-key">{memory.key}</span>
                      <span className={`priority-badge ${getPriorityColor(memory.priority)}`}>
                        P{memory.priority}
                      </span>
                    </div>
                    <div className="memory-value">{memory.value}</div>
                    <div className="memory-meta">
                      <span className="category">{memory.category}</span>
                      <span className="date">{formatDate(memory.created_at)}</span>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}

        {activeTab === 'summary' && (
          <div className="summary-tab">
            <h3>Memory Summary</h3>
            <div className="summary-content">
              {summary ? (
                <div className="summary-text">{summary}</div>
              ) : (
                <p className="empty-state">No summary available.</p>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default MemorySidebar; 