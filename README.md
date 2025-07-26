# Axum API Template

Template completo para APIs REST usando Axum, baseado em boas práticas de arquitetura modular.

## 🚀 Características

- **Axum** - Framework web moderno e performático
- **JWT Authentication** - Sistema completo com access/refresh tokens
- **PostgreSQL + Diesel** - ORM type-safe para Rust
- **Middleware** - Rate limiting, CORS, tratamento de erros
- **Validação** - Validação robusta de dados de entrada
- **Testes** - Estrutura de testes integrados
- **Arquitetura Modular** - Padrão Handler → Service → Repository

## 📋 Pré-requisitos

- **Rust** (1.70+)
- **PostgreSQL** (12+)
- **Diesel CLI**

## ⚙️ Configuração

### 1. Instalar Diesel CLI
```bash
cargo install diesel_cli --no-default-features --features postgres
```

### 2. Configurar Banco de Dados
```bash
# Criar bancos de dados
createdb api_template_dev
createdb api_template_test

# Configurar variáveis de ambiente
cp .env.example .env
```

### 3. Editar arquivo .env
```env
DATABASE_URL=postgresql://username:password@localhost/api_template_dev
TEST_DATABASE_URL=postgresql://username:password@localhost/api_template_test
JWT_ACCESS_SECRET=your-super-secret-access-key-here
JWT_REFRESH_SECRET=your-super-secret-refresh-key-here
```

### 4. Executar Migrations
```bash
diesel migration run
```

### 5. Executar a Aplicação
```bash
cargo run
```

A API estará disponível em `http://localhost:3000`

## 📁 Estrutura do Projeto

```
src/
├── main.rs              # Ponto de entrada
├── app.rs               # Configuração da aplicação
├── config/              # Configurações (banco, etc.)
├── middleware/          # Middleware customizados
├── errors/              # Tratamento de erros padronizado
├── auth/                # Sistema de autenticação JWT
├── user/                # Módulo de usuários
├── db/models/           # Modelos Diesel
├── routes/              # Configuração de rotas
└── utils/               # Utilitários
```

## 🔧 Desenvolvimento

### Adicionando Novos Módulos

1. **Criar estrutura:**
   ```bash
   mkdir src/my_module
   touch src/my_module/{mod.rs,my_handler.rs,my_service.rs,my_repository.rs,my_dto.rs}
   ```

2. **Seguir padrão arquitetural:**
   - **Handler**: Recebe requisições HTTP, valida entrada, chama service
   - **Service**: Lógica de negócio, orquestra chamadas ao repository
   - **Repository**: Acesso aos dados, queries Diesel
   - **DTO**: Estruturas de entrada e saída da API

3. **Registrar rotas** em `routes/mod.rs`

### Migrations

```bash
# Criar nova migration
diesel migration generate create_my_table

# Executar migrations
diesel migration run

# Reverter última migration
diesel migration revert

# Refazer última migration
diesel migration redo
```

### Testes

```bash
# Executar todos os testes
cargo test

# Executar testes específicos
cargo test auth_test

# Executar com logs
cargo test -- --nocapture
```

## 🌐 API Endpoints

### Autenticação (Públicas)
- `POST /auth/register` - Registrar novo usuário
- `POST /auth/login` - Login com email/senha
- `POST /auth/refresh` - Renovar access token
- `POST /auth/forgot-password` - Solicitar reset de senha
- `POST /auth/reset-password` - Resetar senha com token

### Usuário (Protegidas)
- `GET /api/user/profile` - Obter perfil do usuário
- `PUT /api/user/profile` - Atualizar perfil do usuário
- `POST /api/logout` - Logout (revoga tokens)

### Exemplo de Requisição

```bash
# Registrar usuário
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "João",
    "last_name": "Silva",
    "email": "joao@example.com",
    "password": "senha123"
  }'

# Login
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "joao@example.com",
    "password": "senha123"
  }'
```

## 🚀 Produção

### Variáveis de Ambiente
```env
DATABASE_URL=postgresql://user:pass@host:5432/db_prod
JWT_ACCESS_SECRET=your-super-secure-production-secret
JWT_REFRESH_SECRET=your-super-secure-refresh-secret
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
CORS_ORIGIN=https://yourdomain.com
```

### Build e Deploy
```bash
# Build otimizado
cargo build --release

# Executar em produção
./target/release/axum-api-template
```

## 🛠️ Troubleshooting

### Problemas Comuns

**Erro de conexão com banco:**
```bash
# Verificar se PostgreSQL está rodando
pg_isready

# Verificar URL de conexão
echo $DATABASE_URL
```

**Erro de migrations:**
```bash
# Verificar status das migrations
diesel migration list

# Forçar reset (cuidado!)
diesel database reset
```

**Problemas de compilação:**
```bash
# Limpar cache
cargo clean

# Atualizar dependências
cargo update
```


