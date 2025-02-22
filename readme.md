Backend Scheduling

Este backend, desenvolvido em Rust utilizando Axum, Diesel e JWT, fornece uma solução completa para um sistema de agendamento com as seguintes funcionalidades:

Gerenciamento de Dados

Clientes:
CRUD de Clientes: Criação, consulta, atualização e remoção de registros de clientes.
Reservas:
CRUD de Reservas: Gerenciamento dos agendamentos, permitindo que clientes marquem, atualizem e cancelem suas reservas.
Autenticação e Segurança

Registro de Usuários:
Endpoint para cadastro de novos usuários (clientes) com dados básicos (nome, telefone e senha).
A senha é hashada (utilizando Argon2) antes de ser armazenada.
Login & Verificação via SMS:
Os usuários se autenticam informando telefone e senha.
Se as credenciais estiverem corretas, o sistema gera um código SMS temporário (simulado) e o "envia".
O usuário envia esse código para validar sua identidade e, se aceito, recebe um token JWT para acesso às rotas protegidas.
Troca e Recuperação de Senha:
Troca de Senha: Endpoint que permite aos usuários alterarem sua senha, mediante validação da senha atual.
Esqueci a Senha / Reset: Fluxo de recuperação que gera um token temporário (válido por 5 minutos) para redefinir a senha. O token é enviado via SMS (simulado) e utilizado para atualizar a senha.
Gerenciamento Administrativo

Diferenciação de Perfis:
O modelo de usuário possui um campo role que identifica se o usuário é um cliente, um administrador ou o administrador master (proprietário do comércio).
Operações Administrativas:
Adição e Remoção de Administradores: O administrador master pode adicionar novos administradores secundários e remover administradores, garantindo que somente usuários com privilégios de administrador master possam executar essas ações.
Calendário de Agendamentos

Visualização do Calendário:
Endpoint que retorna um calendário com horários disponíveis e ocupados.
Para Administradores: O calendário exibe todos os agendamentos e detalhes (como informações do cliente e horário).
Para Clientes: O calendário mostra apenas os horários disponíveis e marca os horários ocupados como indisponíveis.
Proteção de Rotas com JWT

Após a verificação via SMS, o backend gera um token JWT que contém o ID do usuário e uma data de expiração.
Esse token é utilizado para proteger os endpoints sensíveis, garantindo que apenas usuários autenticados possam acessar as funcionalidades restritas.
Este backend fornece uma base robusta para um sistema de agendamento, permitindo a integração com interfaces web e mobile, e garantindo segurança e controle de acesso com fluxos de autenticação completos e funcionalidades administrativas.