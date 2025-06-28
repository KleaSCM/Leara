# Leara AI Assistant

A comprehensive AI Assistant GUI application with system-level access, built with Electron (TypeScript/React) frontend and Rust backend.

## Architecture

```
Leara/
├── leara-front/     # Electron frontend (TypeScript, React, Vite, SCSS)
└── leara/          # Rust backend (Axum, SQLite, System monitoring)
```

## Features

- **Modern GUI**: Electron-based interface with React, TypeScript, and SCSS
- **System Access**: Rust backend with comprehensive system monitoring
- **Persistent Memory**: SQLite database for conversation history and assistant memory
- **Real-time Communication**: HTTP API between frontend and backend
- **Cross-platform**: Works on Windows, macOS, and Linux

## Tech Stack

### Frontend (`leara-front/`)
- **Electron**: Desktop application framework
- **React**: UI library with TypeScript
- **Vite**: Build tool and dev server
- **SCSS**: Styling with modern CSS features
- **TypeScript**: Type-safe JavaScript

### Backend (`leara/`)
- **Rust**: System programming language
- **Axum**: Web framework
- **SQLite**: Local database with rusqlite
- **sysinfo**: System monitoring library
- **tokio**: Async runtime

## Getting Started

### Prerequisites

- Node.js 18+ and npm
- Rust 1.70+ and Cargo
- Git

### Development Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd Leara
   ```

2. **Setup Frontend**
   ```bash
   cd leara-front
   npm install
   ```

3. **Setup Backend**
   ```bash
   cd ../leara
   cargo build
   ```

4. **Run Development Servers**

   **Terminal 1 - Backend:**
   ```bash
   cd leara
   cargo run
   ```

   **Terminal 2 - Frontend:**
   ```bash
   cd leara-front
   npm run dev
   ```

### Production Build

1. **Build Backend**
   ```bash
   cd leara
   cargo build --release
   ```

2. **Build Frontend**
   ```bash
   cd leara-front
   npm run build
   ```

## Project Structure

### Frontend (`leara-front/`)
```
src/
├── components/     # React components
├── styles/        # SCSS stylesheets
├── types/         # TypeScript type definitions
├── utils/         # Utility functions
├── App.tsx        # Main application component
└── main.tsx       # Application entry point
```

### Backend (`leara/`)
```
src/
├── api/           # API endpoints
├── db/            # Database operations
├── models/        # Data models
├── system/        # System monitoring
├── utils/         # Utility functions
└── main.rs        # Application entry point
```

## API Endpoints

- `GET /health` - Health check
- `POST /api/chat` - Chat with AI assistant
- `GET /api/system/info` - System information
- `GET /api/memory` - Retrieve assistant memory
- `POST /api/memory` - Store assistant memory

## Database Schema

### Conversations
- `id` (TEXT, PRIMARY KEY)
- `title` (TEXT)
- `created_at` (TEXT)
- `updated_at` (TEXT)
- `message_count` (INTEGER)

### Messages
- `id` (TEXT, PRIMARY KEY)
- `conversation_id` (TEXT, FOREIGN KEY)
- `content` (TEXT)
- `sender` (TEXT)
- `timestamp` (TEXT)

### Memory
- `id` (TEXT, PRIMARY KEY)
- `key` (TEXT, UNIQUE)
- `value` (TEXT)
- `category` (TEXT)
- `created_at` (TEXT)
- `updated_at` (TEXT)
- `expires_at` (TEXT, NULLABLE)

## Development

### Adding New Features

1. **Frontend**: Add components in `leara-front/src/components/`
2. **Backend**: Add API endpoints in `leara/src/api/`
3. **Database**: Add migrations in `leara/src/db/migrations.rs`

### Code Style

- **Frontend**: ESLint + Prettier (configure as needed)
- **Backend**: `rustfmt` for Rust code formatting
- **SCSS**: Follow BEM methodology for CSS classes

## Security Considerations

- Electron renderer process has no direct system access
- All system operations go through the Rust backend
- Database is local and encrypted (implement as needed)
- Input validation on both frontend and backend

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Roadmap

- [ ] AI model integration (OpenAI, local models)
- [ ] File system access and management
- [ ] Process monitoring and control
- [ ] Plugin system for extensibility
- [ ] Voice input/output
- [ ] Advanced memory management
- [ ] Multi-language support 