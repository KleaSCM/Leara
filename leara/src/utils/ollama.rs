/*
 * Leara AI Assistant - Ollama Client
 * 
 * This module provides a client for communicating with the Ollama API.
 * Handles model inference, chat completions, and model management.
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
 * File: src/utils/ollama.rs
 * Purpose: Ollama API client implementation
 */


 // Model List
//  openhermes:latest                          
//  hexbenjamin/memgpt-dpo-uncensored:f16      
//  wizard-vicuna-uncensored:13b               
//  qwen2-math:7b                             
//  HammerAI/mythomax-l2:latest               
//  bakllava:latest                            
//  qwen2.5-coder:7b                           
//  qwen2.5-coder:14b                         
//  HammerAI/llama-3-lexi-uncensored:latest    

use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::Result;
use tracing::{info, error};
use serde_json;

/// Ollama API request structure for chat completions
#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    /// The model to use for inference
    pub model: String,
    /// The prompt or message to send to the model
    pub prompt: String,
    /// Optional system message to set context
    pub system: Option<String>,
    /// Optional parameters for model behavior
    pub options: Option<OllamaOptions>,
}

/// Ollama model options for controlling inference behavior
#[derive(Debug, Serialize)]
pub struct OllamaOptions {
    /// Temperature for controlling randomness (0.0 to 1.0)
    pub temperature: Option<f32>,
    /// Maximum number of tokens to generate
    pub num_predict: Option<u32>,
    /// Top-p sampling parameter
    pub top_p: Option<f32>,
    /// Top-k sampling parameter
    pub top_k: Option<u32>,
}

/// Ollama API response structure
#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    /// The generated response text
    pub response: String,
    /// Whether the response is complete
    pub done: bool,
    /// Model information
    pub model: String,
    /// Total tokens used
    pub total_duration: Option<u64>,
    /// Tokens per second
    pub load_duration: Option<u64>,
    /// Prompt evaluation duration
    pub prompt_eval_duration: Option<u64>,
    /// Response generation duration
    pub eval_duration: Option<u64>,
}

/// Client for communicating with Ollama API
pub struct OllamaClient {
    /// HTTP client for making requests
    client: Client,
    /// Base URL for Ollama API (default: http://localhost:11434)
    base_url: String,
}

impl OllamaClient {
    /// Create a new Ollama client with default settings
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://localhost:11434".to_string(),
        }
    }

    /// Create a new Ollama client with custom base URL
    pub fn with_url(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Generate a response using the specified model
    /// 
    /// # Arguments
    /// * `model` - The model name to use (e.g., "hexbenjamin/memgpt-dpo-uncensored:f16")
    /// * `prompt` - The user's message or prompt
    /// * `system` - Optional system message for context
    /// * `options` - Optional model parameters
    /// 
    /// # Returns
    /// * `Ok(String)` - The generated response text
    /// * `Err(anyhow::Error)` - Error if the request fails
    pub async fn generate(
        &self,
        model: &str,
        prompt: &str,
        system: Option<&str>,
        options: Option<OllamaOptions>,
    ) -> Result<String> {
        let request = OllamaRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            system: system.map(|s| s.to_string()),
            options,
        };

        info!("Sending request to Ollama model: {}", model);
        
        let url = format!("{}/api/generate", self.base_url);
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Ollama API error: {}", error_text);
            return Err(anyhow::anyhow!("Ollama API error: {}", error_text));
        }

        // Read the response as text since Ollama returns streaming JSON
        let response_text = response.text().await?;
        let mut full_response = String::new();
        
        // Parse each line as a separate JSON object
        for line in response_text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<OllamaResponse>(line) {
                Ok(ollama_response) => {
                    full_response.push_str(&ollama_response.response);
                    
                    // If this is the final response, break
                    if ollama_response.done {
                        break;
                    }
                }
                Err(e) => {
                    error!("Failed to parse Ollama response line: {}", e);
                    return Err(anyhow::anyhow!("Failed to parse Ollama response: {}", e));
                }
            }
        }
        
        info!("Received response from Ollama model: {}", model);
        Ok(full_response)
    }

    /// Check if a model is available locally
    /// 
    /// # Arguments
    /// * `model` - The model name to check
    /// 
    /// # Returns
    /// * `Ok(bool)` - True if model is available, false otherwise
    pub async fn is_model_available(&self, model: &str) -> Result<bool> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Ok(false);
        }

        #[derive(Deserialize)]
        struct ModelsResponse {
            models: Vec<ModelInfo>,
        }

        #[derive(Deserialize)]
        struct ModelInfo {
            name: String,
        }

        let models_response: ModelsResponse = response.json().await?;
        Ok(models_response.models.iter().any(|m| m.name == model))
    }
}

impl Default for OllamaClient {
    fn default() -> Self {
        Self::new()
    }
} 