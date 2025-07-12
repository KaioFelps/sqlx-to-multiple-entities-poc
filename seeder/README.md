# Seeder
Esse binário é responsável por inserir os dados na aplicação programáticamente para que não seja
necessário fazê-lo manualmente para comprovar o experimento.

## Rodando
`cargo run -p seeder` irá inserir, no banco de dados, as tuplas necessárias para que o código principal execute.

Caso já tenha rodado anteriormente e queira resetar tudo, chame o comando com a flag "reset":
`cargo run -p seeder -F resetar`.
