# Prova de Conceito

Esse programa é uma simples demonstração de como multiplexar tuplas do banco de dados para diferentes
entidades (que sejam derivadas do mesmo modelo do banco de dados).

Neste exemplo, toda tupla da tabela "usuario" (do banco de dados) pode ser interpretada como
[`Usuario`], pois essa é a struct que representa a totalidade do modelo.

Contudo, também pode ser interpretada como um [`Aluno`] se satisfizer as restrições de tipo dessa estrutura
(ter o campo `registro_aluno` e `periodo` como valores não nulos E `cargo` igual a "aluno"), restrições
essas que podem ser garantidas com cláusulas "where".

O mesmo vale para a estrutura [`Professor`], que requer apenas que `cargo` = "professor" | "aluno". 

Além disso, o experimento também tenta buscar entidades e agregações com outras entidades aninhadas com
queries cruas em SQL.

## Conclusão

Saber disso permite um meio elegante para verificações simples de cargos: um serviço de domínio pode
pedir diretamente um [`Aluno`] e prosseguir, ao invés de receber um [`Usuario`] e precisar verificar se
este possui o cargo "aluno".

Não obstante, caso realmente seja necessário realizar asserções mais complexas, basta pedir o [`Usuario`]
de fato e atuar sobre a totalidade do modelo "usuario".

É possível, também, utilizar composição para representar as diferentes classes! Confira a estrutura
[`UsuarioAluno`] que é composta pela entidade [`UsuarioMinimo`] (uma versão mínima da entidade `Usuario`
comum).

Para obter as agregações complexas do banco de dados, foi necessário implementar a trait [`FromRow`]
manualmente, o que também implica queries de SELECT mais complexas e longas, exigindo aliases para cada
atributo de cada entidade aninhada. Confira o [repositório de Vagas](./sqlx_queries/src/vagas.rs) e as
implementações da trait [`FromRow`] para as entidades de [projeto] e [vaga](./poc_core/src/entidades/vaga.rs).

Por meio desse experimento, também encontramos uma forma mais elegante de lidar com números sem sinal nas
entidades que, no banco de dados, precisam ser um inteiro de 16 bits, no mínimo. Veja a estrutura
[NullableU8](./poc_core/src/types.rs) e as implementações que foram necessárias para que ela pudesse ser
utilizada como está sendo na entidade de [projeto] e no [seu repositório](./sqlx_queries/src/projetos.rs).

[`UsuarioAluno`]: ./src/entidades/aluno.rs
[`UsuarioMinimo`]: ./src/entidades/usuario.rs
[`FromRow`]: https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html
[projeto]: ./poc_core/src/entidades/projeto.rs

## Como Rodar o Programa
### Requisitos
- [docker](https://docs.docker.com/get-started/get-docker/)
- [rust + cargo](https://www.rust-lang.org/tools/install)
- [sqlx cli](https://crates.io/crates/sqlx-cli)

```bash
cp .env.sample .env
docker compose up -d # inicializa o Postgres
sqlx migrate run # roda as migrações pra montar o banco de dados
cargo run -p seeder -F resetar # insere algumas tuplas no banco de dados, necessário para rodar a prova de conceito
```

### Rodando
Basta utilizar o comando `cargo run` para baixar as dependências, compilar o programa e rodá-lo.

[`Usuario`]: src/entidades/usuario.rs
[`Aluno`]: src/entidades/aluno.rs
[`Professor`]: src/entidades/professor.rs
