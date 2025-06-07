-- Add migration script here
CREATE TYPE cargo_e AS ENUM ('aluno', 'professor', 'administrador');

CREATE TABLE "usuario" (
    id                  UUID            NOT NULL    DEFAULT gen_random_uuid(),
    nome                VARCHAR(200)    NOT NULL,
    email               VARCHAR(200)    NOT NULL,
    senha_hash          VARCHAR(60)     NOT NULl,
    curriculo_lattes    VARCHAR(300),
    cargo               cargo_e         NOT NULL,
    ultimo_login_em     TIMESTAMP,
    atualizado_em       TIMESTAMP,
    criado_em           TIMESTAMP       NOT NULL    DEFAULT now(),

    --- específicos de entidades
    registro_aluno      VARCHAR(100),
    periodo             SMALLINT,

    CONSTRAINT usuario_pk       PRIMARY KEY (id),
    CONSTRAINT ra_unique        UNIQUE (registro_aluno),
    CONSTRAINT email_unique     UNIQUE (email)
);
