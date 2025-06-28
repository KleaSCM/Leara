declare global {
  interface Window {
    electronAPI: {
      apiRequest: (endpoint: string, method: string, data?: any) => Promise<any>;
      getSystemInfo: () => Promise<any>;
      minimize: () => Promise<void>;
      maximize: () => Promise<void>;
      close: () => Promise<void>;
    };
  }
}

export {}; 