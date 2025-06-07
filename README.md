# Prova de Conceito

Esse programa é uma simples demonstração de como multiplexar tuplas do banco de dados para diferentes
entidades (que sejam derivadas do modelo do banco de dados).

Neste exemplo, toda tupla da tabela "usuario" (do banco de dados) pode ser interpretada como
[`Usuario`], pois essa é a struct que representa a totalidade do modelo.

Contudo, também pode ser interpretada como um [`Aluno`] se satisfizer as restrições de tipo dessa estrutura
(ter o campo `registro_aluno` e `periodo` como valores não nulos E `cargo` igual a "aluno"), restrições essas que podem ser garantidas
com cláusulas "where".

O mesmo vale para a estrutura [`Professor`], que requer apenas que `cargo` = "professor" | "aluno". 

## Conclusão

Saber disso permite um meio elegante para verificações simples de cargos: um serviço de domínio pode
pedir diretamente um [`Aluno`] e prosseguir, ao invés de receber um [`Usuario`] e precisar verificar se
este possui o cargo "aluno".

Não obstante, caso realmente seja necessário realizar asserções mais complexas, basta pedir o [`Usuario`]
de fato e atuar sobre a totalidade do modelo "usuario".

## Como Rodar o Programa
### Requisitos
- [docker](https://docs.docker.com/get-started/get-docker/)
- [rust + cargo](https://www.rust-lang.org/tools/install)

```bash
cp .env.sample .env
docker compose up -d # inicializa o Postgres
```

Utilize um programa como [BeeKeeper](https://www.beekeeperstudio.io/) para adicionar algumas tuplas na
tabela "usuario" antes de rodar, caso contrário o programa entrará em pânico: ele espera ao menos 1
usuário *aluno* válido e ao menos 1 usuário *professor* válido.

### Rodando
Basta utilizar o comando `cargo run` para baixar as dependências, compilar o programa e rodá-lo.

[`Usuario`]: src/entidades/usuario.rs
[`Aluno`]: src/entidades/aluno.rs
[`Professor`]: src/entidades/professor.rs
