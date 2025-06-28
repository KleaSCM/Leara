/*
 * Leara AI Assistant - API Utilities
 * 
 * This module provides TypeScript functions for communicating with the
 * Leara AI Assistant backend API endpoints.
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
 * File: src/utils/api.ts
 * Purpose: API communication utilities
 */

const API_BASE = 'http://localhost:3000/api';

// Types for API responses
export interface Memory {
  id: number;
  key: string;
  value: string;
  category: string;
  priority: number;
  metadata?: any;
  created_at: string;
  updated_at: string;
  expires_at?: string;
  is_active: boolean;
}

export interface MemoryResponse {
  memories: Memory[];
  total: number;
}

export interface MemoryOperationResponse {
  success: boolean;
  message: string;
}

export interface Task {
  id: number;
  title: string;
  description?: string;
  status: string;
  priority: number;
  due_date?: string;
  created_at: string;
  updated_at: string;
  completed_at?: string;
  context?: string;
  tags?: string;
}

export interface TaskResponse {
  tasks: Task[];
  total: number;
}

export interface SessionContext {
  id: number;
  session_id: string;
  context_key: string;
  context_value: string;
  created_at: string;
  updated_at: string;
}

export interface SessionContextResponse {
  contexts: SessionContext[];
  total: number;
}

export interface ChatResponse {
  message: string;
  conversation_id: string;
  timestamp: string;
  context?: string;
}

// Memory API functions
export async function getMemories(params: any = {}): Promise<MemoryResponse> {
  const queryString = new URLSearchParams(params).toString();
  const url = queryString ? `${API_BASE}/memory?${queryString}` : `${API_BASE}/memory`;
  
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to fetch memories: ${response.statusText}`);
  }
  return response.json();
}

export async function addMemory(data: {
  key: string;
  value: string;
  category?: string;
  priority?: number;
  metadata?: any;
  context?: string;
  expires_at?: string;
}): Promise<MemoryOperationResponse> {
  const response = await fetch(`${API_BASE}/memory`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to add memory: ${response.statusText}`);
  }
  return response.json();
}

export async function searchMemories(query: string): Promise<MemoryResponse> {
  const response = await fetch(`${API_BASE}/memory/search`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ query }),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to search memories: ${response.statusText}`);
  }
  return response.json();
}

export async function getMemorySummary(): Promise<{ summary: string }> {
  const response = await fetch(`${API_BASE}/memory/summary`);
  
  if (!response.ok) {
    throw new Error(`Failed to get memory summary: ${response.statusText}`);
  }
  return response.json();
}

// Task API functions
export async function getTasks(params: any = {}): Promise<TaskResponse> {
  const queryString = new URLSearchParams(params).toString();
  const url = queryString ? `${API_BASE}/tasks?${queryString}` : `${API_BASE}/tasks`;
  
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to fetch tasks: ${response.statusText}`);
  }
  return response.json();
}

export async function addTask(data: {
  title: string;
  description?: string;
  priority?: number;
  due_date?: string;
  context?: string;
  tags?: string;
}): Promise<Task> {
  const response = await fetch(`${API_BASE}/tasks`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to add task: ${response.statusText}`);
  }
  return response.json();
}

export async function updateTaskStatus(taskId: number, status: string): Promise<MemoryOperationResponse> {
  const response = await fetch(`${API_BASE}/tasks/${taskId}/status`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ status }),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to update task status: ${response.statusText}`);
  }
  return response.json();
}

// Session Context API functions
export async function storeSessionContext(data: {
  session_id: string;
  context_key: string;
  context_value: string;
}): Promise<MemoryOperationResponse> {
  const response = await fetch(`${API_BASE}/context`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to store session context: ${response.statusText}`);
  }
  return response.json();
}

export async function getSessionContext(sessionId: string): Promise<SessionContextResponse> {
  const response = await fetch(`${API_BASE}/context/${sessionId}`);
  
  if (!response.ok) {
    throw new Error(`Failed to get session context: ${response.statusText}`);
  }
  return response.json();
}

// Chat API functions
export async function sendChatMessage(data: {
  message: string;
  context?: string;
  session_id?: string;
}): Promise<ChatResponse> {
  const response = await fetch(`${API_BASE}/chat`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to send chat message: ${response.statusText}`);
  }
  return response.json();
}

export async function sendMemoryQuery(data: {
  message: string;
  context?: string;
  session_id?: string;
}): Promise<ChatResponse> {
  const response = await fetch(`${API_BASE}/chat/memory`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to send memory query: ${response.statusText}`);
  }
  return response.json();
}

export async function getConversationSummary(data: {
  session_id?: string;
}): Promise<ChatResponse> {
  const response = await fetch(`${API_BASE}/chat/summary`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to get conversation summary: ${response.statusText}`);
  }
  return response.json();
}

// Health check
export async function checkHealth(): Promise<{ status: string }> {
  const response = await fetch(`${API_BASE}/health`);
  
  if (!response.ok) {
    throw new Error(`Health check failed: ${response.statusText}`);
  }
  return response.json();
} 